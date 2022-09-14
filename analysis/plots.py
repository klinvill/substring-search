import pandas as pd
from pathlib import Path
import seaborn as sns
import matplotlib.pyplot as plt


def load_sample_data(path: Path) -> pd.DataFrame:
    df = pd.DataFrame()
    csv_files = path.glob("**/new/raw.csv")
    for f in csv_files:
        df = pd.concat([df, pd.read_csv(f)])

    # Separate out "value" column into file and k values
    df[['file', 'k']] = df['value'].str.rsplit('_', n=1, expand=True)

    # Simple benchmark used k=5
    df.k[df['file'] == 'simple'] = 5

    # k values should be integers
    df['k'] = df['k'].astype('int')

    # convert time per sample to time per iteration (more information about what sample_measured_value is is available
    # at https://bheisler.github.io/criterion.rs/book/user_guide/csv_output.html)
    df['iter_time'] = df['sample_measured_value'] / df['iteration_count']

    # convert time measurements into milliseconds
    df['iter_time'] = df['iter_time'] / 1e6

    return df


def plot_hashes():
    data_path = "../target/criterion/Hashes"
    hash_functions = [
        "sip",
        "fx",
        "rolling_adler",
        "rolling_poly",
    ]

    df = pd.DataFrame()
    # Loads all the data files into a single dataframe
    for func in hash_functions:
        path = Path(data_path, func)
        df = pd.concat([df, load_sample_data(path)])

    for f in df['file'].unique():
        if f == "simple":
            plt.figure()
            ax = sns.boxplot(data=df[df['file'] == f], x="function", y="iter_time")
            ax.set_ylabel("ms")
            ax.set_title(f"{f}, k=5")
            ax.legend(bbox_to_anchor=(1, 1), loc="center left")
            plt.tight_layout()
            plt.savefig(f"hashes_{f}.png")
        else:
            sns.relplot(data=df[df['file'] == f], x="k", y="iter_time", hue="function", kind="line",
                        legend="full", col="file")\
                .set_ylabels("ms")\
                .savefig(f"hashes_{f}.png")


def plot_implementations():
    data_path = "../target/criterion/Substring"
    implementations = [
        "naive_substring",
        "naive_prereserve_substring",
        "naive_prereserve_iter_substring",
        "naive_prereserve_iter_fx_substring",
        "naive_prereserve_iter_fx_shorter_substring",
        "naive_prereserve_iter_rolling_adler_shorter_substring",
        "naive_prereserve_iter_rolling_poly_shorter_substring",
        "alternate_prereserve_iter_fx_substring",
    ]

    df = pd.DataFrame()
    # Loads all the data files into a single dataframe
    for impl in implementations:
        path = Path(data_path, impl)
        df = pd.concat([df, load_sample_data(path)])

    for f in df['file'].unique():
        if f == "simple":
            plt.figure()
            ax = sns.boxplot(data=df[df['file'] == f], x="function", y="iter_time")
            ax.set_ylabel("ms")
            ax.set_title(f"{f}, k=5")
            ax.legend(bbox_to_anchor=(1, 1), loc="center left")
            plt.tight_layout()
            plt.savefig(f"impls_{f}.png")
        else:
            data = df[df['file'] == f]
            if not data[data['function'] == "naive_prereserve_iter_rolling_adler_shorter_substring"].empty:
                with_data = data
                sns.relplot(data=with_data, x="k", y="iter_time", hue="function", kind="line",
                            legend="full", col="file") \
                    .set_ylabels("ms") \
                    .savefig(f"impls_{f}_with_adler.png")

            without_data = data[data['function'] != "naive_prereserve_iter_rolling_adler_shorter_substring"]
            sns.relplot(data=without_data, x="k", y="iter_time", hue="function", kind="line",
                        legend="full", col="file") \
                .set_ylabels("ms") \
                .savefig(f"impls_{f}_without_adler.png")


def plot_collisions():
    # This data is small enough that it was just copied from the output of running the check_collisions function
    unique_hashes = {
        "war_and_peace_tolstoy.txt": {"sip": 3078478, "rolling_poly": 3078478, "rolling_adler": 1333723, "fx": 3078478},
        "monkeypox-genome.txt": {"sip": 242651, "rolling_poly": 242651, "rolling_adler": 113818, "fx": 242651},
    }

    entries = []
    for file, data in unique_hashes.items():
        for func, count in data.items():
            entries.append((file, func, count))
    df = pd.DataFrame(entries, columns=["file", "function", "unique_hashes"])

    # Represent number of unique hashes in millions
    df['unique_hashes'] = df['unique_hashes'] / 1e6

    plt.figure()
    ax = sns.barplot(df, x="file", y="unique_hashes", hue="function")
    ax.set_ylabel("unique hashes (in millions)")
    plt.savefig(f"collisions.png")


if __name__ == "__main__":
    plot_hashes()
    plot_implementations()
    plot_collisions()
