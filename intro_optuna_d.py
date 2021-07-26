import optuna
import subprocess


def objective(trial):
    score = 0
    x = trial.suggest_uniform('x', -10, 10)
    with open("./in/input00.txt", "a") as f:
        f.write(f"{x}")
    with open("./in/input00.txt", "r") as f:
        res = subprocess.run(["cargo",
                              "run",
                              "--release",
                              "--bin",
                              "intro-heuristics-d"],
                             stdin=f,
                             capture_output=True,
                             text=True)
        out = res.stdout.split()
        score = float(out[0]) if out else 0
    subprocess.run(["sed", "-i", "$d", "./in/input00.txt"])
    print('x: %1.3f, score: %1.3f, out: %r' % (x, score, out))
    return score


study = optuna.create_study(direction="maximize")
study.optimize(objective, n_trials=100)
print(study.best_params)
print(study.best_value)
