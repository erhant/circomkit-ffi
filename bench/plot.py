import matplotlib.pyplot as plt
import numpy as np

# Data
multipliers = [
    "3",
    "30",
    "300",
    "3000",
    "30000",
    "300000",
]
arkworks_times = [2.08, 4.76, 30.45, 280.94, 2812.81, 28472.77]  # in ms
snarkjs_times = [91.00, 97.81, 186.71, 697.93, 4491.08, 46463.74]  # in ms

# Positions for the bars
x = np.arange(len(multipliers))  # the label locations
width = 0.35  # the width of the bars

# Plotting
fig, ax = plt.subplots(figsize=(12, 8))

# Bars for Arkworks and SnarkJS
bar1 = ax.bar(
    x - width / 2, arkworks_times, width, label="Arkworks", color="blue", alpha=0.7
)
bar2 = ax.bar(
    x + width / 2, snarkjs_times, width, label="SnarkJS", color="orange", alpha=0.7
)

# Add labels, title, and custom x-axis tick labels
ax.set_xlabel("Circuit Size (multiplier_N)", fontsize=14)
ax.set_ylabel("Proving Time (ms, log scale)", fontsize=14)
ax.set_yscale("log")  # Set y-axis to logarithmic scale
ax.set_title("Proving Time Comparison: Arkworks vs SnarkJS", fontsize=16, pad=20)
ax.set_xticks(x)
ax.set_xticklabels(multipliers, rotation=45, ha="right", fontsize=12)
ax.legend(fontsize=12)

# Add grid lines for better readability
ax.grid(True, linestyle="--", alpha=0.6)


# Annotate the bars with their values
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
            fontsize=10,
        )


annotate_bars(bar1)
annotate_bars(bar2)

# Adjust layout to prevent clipping of labels
plt.tight_layout()

# Show the plot
plt.savefig("plot.png")
