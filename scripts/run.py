from __future__ import annotations

import os
import glob
import json
import hashlib
import multiprocessing as mp
import dataclasses
import argparse
import subprocess
import functools
import itertools
import tqdm
import time


@dataclasses.dataclass
class Config:
    instanceTimes: dict[str, int]
    algorithms: list[str]
    outputFolder: str
    instancesFolder: str
    cmdBaseArgs: str
    repeats: int
    startSeed: int
    nodeSwap: int = 1
    edgeSwap: int = 1
    cwd: str = os.getcwd()
    gridParams: dict[str, list[float]] = dataclasses.field(default_factory=dict)

    @staticmethod
    def from_json(jsonFile: str) -> Config:
        with open(jsonFile, "r") as f:
            data = json.load(f)
            return Config(**data)


@dataclasses.dataclass
class RunSpec:
    rep: int
    algorithm: str
    instance: str
    time: int
    params: dict[str, float] = dataclasses.field(default_factory=dict)

    @staticmethod
    def from_config(config: Config) -> list[RunSpec]:
        run_specs = []
        for rep in range(config.repeats):
            for algorithm in config.algorithms:
                for instance, time in config.instanceTimes.items():
                    all_grid_configs = [
                        dict(zip(config.gridParams, p))
                        for p in itertools.product(*config.gridParams.values())
                    ]
                    for grid_config in all_grid_configs:
                        run_specs.append(
                            RunSpec(
                                rep + config.startSeed,
                                algorithm,
                                instance,
                                time,
                                grid_config,
                            )
                        )
        return run_specs

    def get_hash(self) -> str:
        return hashlib.md5(
            f"{self.rep}_{self.algorithm}_{self.instance}_{self.time}_{self.params}".encode()
        ).hexdigest()


def run_experiment(
    config: Config,
    run_spec: RunSpec,
) -> bool:
    os.chdir(config.cwd)
    out_file = (
        f"PARTIAL_{run_spec.instance}_{run_spec.algorithm}_{run_spec.get_hash()}.json"
    )
    full_out_file = os.path.join(config.outputFolder, out_file)
    instance_file = os.path.join(config.instancesFolder, run_spec.instance) + ".atsp"
    time_limit_arg = f"-m {run_spec.time}" if run_spec.time > 0 else ""
    cmd_args = f"{config.cmdBaseArgs} -i {instance_file} -a {run_spec.algorithm} {time_limit_arg} -o {full_out_file} -s {run_spec.rep} -t --node-swap {config.nodeSwap} --edge-swap {config.edgeSwap}"
    for key, val in run_spec.params.items():
        cmd_args += f" {key} {val}"

    res = subprocess.run(
        ["cargo", *cmd_args.split()],
        capture_output=True,
        text=True,
    )
    if res.returncode == 0:
        return True

    error_file = f"ERROR_{run_spec.instance}_{run_spec.algorithm}_{run_spec.rep}.txt"
    full_error_file = os.path.join(config.outputFolder, error_file)
    with open(full_error_file, "w") as f:
        f.write(res.stdout)
        f.write(res.stderr)
    return False


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Run experiments")
    parser.add_argument(
        "--config", "-c", type=str, help="Path to the configuration file"
    )
    parser.add_argument(
        "--output", "-o", type=str, help="Name of the output file", default="output"
    )
    parser.add_argument(
        "--jobs",
        "-j",
        type=int,
        help="Number of jobs to run in parallel",
        default=mp.cpu_count(),
    )
    args = parser.parse_args()
    config = Config.from_json(args.config)
    run_specs = RunSpec.from_config(config)

    total_count = len(run_specs)
    print(f"Running {total_count} experiments")

    res = subprocess.run(
        ["cargo", "build", "--manifest-path", "./atsp_solver/Cargo.toml", "--release"],
        capture_output=True,
        text=True,
    )

    if res.returncode != 0:
        print("Error building the project")
        exit(1)

    print("Removing old partial output files")
    for file in glob.glob(f"{config.outputFolder}/PARTIAL_*"):
        os.remove(file)

    job = functools.partial(
        run_experiment,
        config,
    )
    print(f"Running experiments with {args.jobs} processes")
    time_start = time.perf_counter()
    with mp.Pool(processes=args.jobs) as pool:
        with tqdm.tqdm(total=total_count) as pbar:
            for _ in pool.imap_unordered(job, run_specs):
                pbar.update()

    print(f"Finished in {time.perf_counter() - time_start:.2f} seconds")

    print("Merging partial output files")
    all_json = []
    for file in glob.glob(f"{config.outputFolder}/PARTIAL_*"):
        with open(file, "r") as f:
            all_json.append(json.load(f))
        os.remove(file)

    with open(f"{config.outputFolder}/{args.output}.json", "w") as f:
        json.dump(all_json, f)

    print(f"Output written to {config.outputFolder}/{args.output}.json")
    print("Done")
