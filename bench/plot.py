import matplotlib.pyplot as plt
import numpy as np

# x-axis
multipliers = ["3", "30", "300", "3000", "30000", "300000"]

# data taken from raw readme
bun_ffi_times = [2.08, 4.76, 30.45, 280.94, 2812.81, 28472.77]  # Arkworks on Bun
bun_times = [91.00, 97.81, 186.71, 697.93, 4491.08, 46463.74]  # SnarkJS on Bun
ffi_rs_times = [2.10, 4.77, 36.10, 275.04, 2826.17, 28184.95]  # Arkworks on Node
node_times = [17.82, 12.62, 30.70, 148.49, 1126.89, 27709.03]  # SnarkJS on Node

# Positions for the bars
x = np.arange(len(multipliers))  # the label locations
width = 0.2  # narrower width to fit 4 bars

# Plotting
fig, ax = plt.subplots(figsize=(14, 8))

# bars for datasets
bar1 = ax.bar(
    x - 1.5 * width,
    bun_times,
    width,
    label="bun (snarkjs)",
    color="tan",
    alpha=0.7,
)
bar2 = ax.bar(
    x - 0.5 * width,
    bun_ffi_times,
    width,
    label="bun (ffi: arkworks)",
    color="goldenrod",
    alpha=0.7,
)
bar3 = ax.bar(
    x + 0.5 * width,
    node_times,
    width,
    label="node (snarkjs)",
    color="greenyellow",
    alpha=0.7,
)
bar4 = ax.bar(
    x + 1.5 * width,
    ffi_rs_times,
    width,
    label="node (ffi: arkworks)",
    color="green",
    alpha=0.7,
)

# labels & title
ax.set_xlabel("Circuit Size (N in multiplier_N)", fontsize=14)
ax.set_ylabel("Proving Time (ms, log-scale)", fontsize=14)
ax.set_yscale("log")  # Set y-axis to logarithmic scale
ax.set_title(
    "Proving Time Comparison across Runtimes and Implementations", fontsize=16, pad=20
)
ax.set_xticks(x)
ax.set_xticklabels(multipliers, rotation=45, ha="right", fontsize=12)
ax.legend(fontsize=12)
ax.grid(True, axis="y", linestyle="--", alpha=0.6)


def annotate_bars(bars):
    for bar in bars:
        height = bar.get_height()
        ax.annotate(
            f"{height:.2f}",
            xy=(bar.get_x() + bar.get_width() / 2, height),
            xytext=(0, 3),  # 3 points vertical offset
            textcoords="offset points",
            ha="center",
            va="bottom",
            fontsize=8,
            rotation=45,
        )


annotate_bars(bar1)
annotate_bars(bar2)
annotate_bars(bar3)
annotate_bars(bar4)

# adjust layout to prevent clipping of labels
plt.tight_layout()

# save the plot
plt.savefig("plot.png", dpi=300)
plt.close()
