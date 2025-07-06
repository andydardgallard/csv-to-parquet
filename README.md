# csv-to-parquet
A fast and parallel CSV-to-Parquet converter written in Rust. Supports multi-threaded processing, progress tracking, and clean output directory handling.



## 📌 Features

✅ Converts .txt files containing CSV data into Apache Parquet format.

🚀 Parallel processing using rayon.

🔁 Progress bars with indicatif.

🧹 Automatically cleans and prepares the output directory.

📊 Time tracking and performance reporting.

📂 Supports CLI arguments for input/output paths and number of threads.



## 🧰 Requirements

Make sure you have installed:
- Rust (latest stable version)
- Cargo (comes with Rust)
Install Rust via rustup, if needed.



## 🧱 Installation
1. Clone the repository:
   
#bash
- git clone https://github.com/your-username/csv_to_parquet.git 
- cd csv_to_parquet
  
3. Build the project:
#bash
- cargo build --release

5. Run the program:
#bash
- cargo run -- -i <input_dir> -o <output_dir> [-t <threads>]



## ⚙️ Usage

Example:

#bash

1. cargo run -- -i ../../../Python/Backtester/Tickers/Si -o ../Tickers/Si -t 4

CLI Arguments:

Flag	Description:	

-i"	 "--input	Input directory with .txt CSV files

-o"	 "--output	Output directory for Parquet files

-t"	 "--threads	Number of threads to use (optional)



## 📁 Project Structure

src/

├── main.rs             # Entry point

├── cli.rs                # Command-line argument parsing

├── file_processing.rs    # File listing and directory management

├── csv_to_parquet.rs     # CSV → DataFrame → Parquet conversion logic

├── progress.rs           # Multi-threaded file processing with progress bars

└── utils.rs              # Utility functions (e.g., thread pool setup)



## 🧪 Verify Output (Python)

You can verify the generated Parquet files using Python:

Install dependencies:

#bash

1 pip install pyarrow pandas

Sample script: open_parquet.py

Run verification:

#bash

1 python open_parquet.py ../Tickers/Si/



## 💬 Support

If you have any questions or suggestions, feel free to open an issue on GitHub.



