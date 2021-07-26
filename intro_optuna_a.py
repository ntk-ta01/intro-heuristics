import optuna
import subprocess


def objective(trial):
    score = 0
    start_temp = trial.suggest_float('start_temp', 1000, 100000)
    end_temp = trial.suggest_float('end_temp', 0.01, 1000)
    LOOP_TIME = 5
    for i in range(LOOP_TIME):
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
            score += float(out[0]) if out else 0
        subprocess.run(["sed", "-i", "$d", f"./in/input0{i}.txt"])
        subprocess.run(["sed", "-i", "$d", f"./in/input0{i}.txt"])
    # print('start temp: %1.3f,end temp: %1.3f score: %1.3f' %
    #       (start_temp, end_temp, score / 5.0))
    return score / 5.0


study = optuna.create_study(direction="maximize")
study.optimize(objective, n_trials=100)
print(study.best_params)
print(study.best_value)
