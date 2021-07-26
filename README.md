# intro-heuristics
https://atcoder.jp/contests/intro-heuristics を解いた．

Optunaを使うのが目的で解き直しており，d.rsは(x - 2)^2のxが[-10,10]における最大化を行う目的で用意している．

joblibによるテストケースの並列処理まで書いているが，各温度ごとの計算まで並列処理したければDBを使用する必要がある．

Optunaの使い方を見たければ，intro_optuna_d.py -> intro_optuna_a.py -> intro_oputuna_a_parallel.pyの順に読むとまあまあわかる気がする．
それぞれ、subprocessを使った入力ファイルを読み込んだRustの実行，複数入力ファイルでのRustの実行，並列処理，と段階的に難しいことを行った．

optunaのAPIリファレンス：https://optuna.readthedocs.io/en/stable/reference/index.html
