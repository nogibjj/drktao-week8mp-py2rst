from main import denirostats
from main import denirohist


def test_deniro():
    assert denirostats("deniro.csv").iloc[0, 0] == 58.20
    assert denirostats("deniro.csv").iloc[0, 1] == 65.00
    assert denirostats("deniro.csv").iloc[0, 2] == 28.07


def test_viz():
    denirohist("deniro.csv")
