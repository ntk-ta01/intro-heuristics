import optuna
import subprocess
from joblib import Parallel, delayed


def test(i, start_temp, end_temp):
    score = 0
    with open(f"./in/input0{i}.txt", "a") as f:
        f.write(f"{start_temp}\n")
        f.write(f"{end_temp}")
    with open(f"./in/input0{i}.txt", "r") as f:
        res = subprocess.run(["cargo",
                              "run",
                              "--quiet",
                              "--release",
                              "--bin",
                              "intro-heuristics-a"],
                             stdin=f,
                             capture_output=True,
                             text=True)
        out = res.stdout.split()
        score = float(out[0]) if out else 0
    subprocess.run(["sed", "-i", "$d", f"./in/input0{i}.txt"])
    subprocess.run(["sed", "-i", "$d", f"./in/input0{i}.txt"])
    # print('%r: start temp: %1.3f,end temp: %1.3f score: %1.3f, res: %r' %
    #       (i, start_temp, end_temp, score, res.returncode))
    return score


def objective(trial):
    start_temp = trial.suggest_float('start_temp', 1000, 100000)
    end_temp = trial.suggest_float('end_temp', 0.01, 1000)
    LOOP_TIME = 5
    all_test_case_result = Parallel(
        n_jobs=-1)(
        [delayed(test)(i, start_temp, end_temp) for i in range(LOOP_TIME)])
    return sum(all_test_case_result) / LOOP_TIME


study = optuna.create_study(direction="maximize")
study.optimize(objective, n_trials=100)
print(study.best_params)
print(study.best_value)
