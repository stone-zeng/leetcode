"""175. Combine Two Tables
https://leetcode.com/problems/combine-two-tables/
"""

import pandas as pd


def combine_two_tables(person: pd.DataFrame, address: pd.DataFrame) -> pd.DataFrame:
    df = pd.merge(person, address, "left")
    df.drop(columns=["personId", "addressId"], inplace=True)
    df.where(pd.notna(df), None, inplace=True)
    return df


if __name__ == "__main__":
    person = pd.DataFrame(
        [[1, "Wang", "Allen"], [2, "Alice", "Bob"]],
        columns=["personId", "firstName", "lastName"],
    ).astype({"personId": "Int64", "firstName": "object", "lastName": "object"})
    address = pd.DataFrame(
        [[1, 2, "New York City", "New York"], [2, 3, "Leetcode", "California"]],
        columns=["addressId", "personId", "city", "state"],
    ).astype(
        {"addressId": "Int64", "personId": "Int64", "city": "object", "state": "object"}
    )
    print(combine_two_tables(person, address))
