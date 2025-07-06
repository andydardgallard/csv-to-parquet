import sys
import os
import pyarrow.parquet as pq
import pandas as pd

if __name__ == "__main__":
    list_dir = os.listdir(sys.argv[1])
    if len(sys.argv) != 2:
        print("Invalid number of arguments! Usage: Path to folder to .parquet files!")
    else:
        for file in list_dir:
            if sys.argv[1].endswith('/'):
                file_path = f'{sys.argv[1]}{file}'
            else:
                file_path = f'{sys.argv[1]}/{file}'

            # Open files with pandas
            df = pd.read_parquet(file_path)
            df = df.set_index("datetime_str")
            print(f'Open file {file} with pandas')
            print(df.head())
            print('\n')

            # Open files with pyarrow
            table = pq.read_table(file_path)
            print(f'Open file {file} with pyarrow')
            print(table.schema)

