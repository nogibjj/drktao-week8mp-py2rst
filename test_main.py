from main import denirostats


def test_deniro():
    assert denirostats("deniro.csv").iloc[0, 0] == 58.20
    assert denirostats("deniro.csv").iloc[0, 1] == 65.00
