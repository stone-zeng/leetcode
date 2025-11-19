"""176. Second Highest Salary
https://leetcode.com/problems/second-highest-salary/
"""

import pandas as pd


def second_highest_salary(employee: pd.DataFrame) -> pd.DataFrame:
    df = employee.drop_duplicates(["salary"]).nlargest(2, "salary")[1:2]
    print(f"{df.shape[0] = }")
    return pd.DataFrame(
        {"SecondHighestSalary": [None] if df.empty else df["salary"].values}
    )


if __name__ == "__main__":
    for data in [
        [[1, 100]],
        [[1, 100], [2, 100]],
        [[1, 100], [2, 100], [3, 50]],
        [[1, 100], [2, 200], [3, 300], [4, 400]],
    ]:
        employee = pd.DataFrame(data, columns=["id", "salary"]).astype(
            {"id": "int64", "salary": "int64"}
        )
        print(second_highest_salary(employee))
