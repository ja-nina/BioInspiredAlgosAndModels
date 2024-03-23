from __future__ import annotations

import os
import glob
import json
import multiprocessing as mp
import dataclasses
import argparse
import subprocess
import functools
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
    cwd: str = os.getcwd()

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

    @staticmethod
    def from_config(config: Config) -> list[RunSpec]:
        run_specs = []
        for rep in range(1, config.repeats + 1):
            for algorithm in config.algorithms:
                for instance, time in config.instanceTimes.items():
                    run_specs.append(RunSpec(rep, algorithm, instance, time))
        return run_specs


def run_experiment(
    config: Config,
    run_spec: RunSpec,
) -> bool:
    os.chdir(config.cwd)
    out_file = f"PARTIAL_{run_spec.instance}_{run_spec.algorithm}_{run_spec.rep}.json"
    full_out_file = os.path.join(config.outputFolder, out_file)
    instance_file = os.path.join(config.instancesFolder, run_spec.instance) + ".atsp"
    cmd_args = f"{config.cmdBaseArgs} -i {instance_file} -a {run_spec.algorithm} -m {run_spec.time} -o {full_out_file} -s {run_spec.rep} -t"
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
    print(f"Running experiments with {mp.cpu_count()} processes")
    time_start = time.perf_counter()
    with mp.Pool(processes=mp.cpu_count()) as pool:
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
