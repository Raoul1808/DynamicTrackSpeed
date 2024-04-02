# Dynamic Track Speed

Scroll Velocities in SRXD. Finally!

## How to use

The mod is made to be as simple to use as possible. You will need:
- A text editor
- The chart (an the chart's file name)

To get started adding speed triggers, open your customs directory and create a new file with the same name as the SRTB file you want to add speed triggers do, but end it with the `.speeds` extension.

For example if you want to add triggers to `NULCTRL.srtb`, create a new file called `NULCTRL.speeds` **right next to `NULCTRL.srtb`**. They must be in the same directory.

Next, open the `.speeds` file. You might want to use the editor for this section.

Speed triggers in the `speeds` format use this format:
```
# You can use comments like this
# Triggers follow this format:
<time> <speed multiplier> [interpolate]

# Examples
# Create a double speed trigger at timestamp 14.5 seconds
14.5 2 false

# Create a half speed trigger at timestamp 18 seconds that interpolates to the next trigger
18 0.5 true

# You don't have to specify whether to interpolate or not.
# In case you don't, the game automatically interprets this to be false (default value of a boolean)
```

Once the file is saved, go back to the chart listing and go to your chart. If you are already on your chart, go to another and come back. Every time you reload the chart preview, the mod will reload speeds, making quick editing convenient.

## Integration tool

Due to SpinShare uploading limitations, you can integrate the speed triggers directly in your SRTB by using a convenient little tool you can find with the mod download.

The tool is rather self-explanatory. You can integrate your speeds file in your chart or extract an existing chart's speeds file in case you want to analyze the speed triggers.

## Building the mod

Prerequisites:
- .NET Framework SDK
- Visual Studio or Rider (unless you're familiar with command-line MSBuild)

Steps:
1. Clone this repo
2. Create a symlink to your spin directory named `srxd-dir` and located right next to the `.sln` file
3. Open the solution
4. Build the mod
5. Enjoy!

**This mod depends on Newtonsoft.Json, make sure to include it in your plugins folder!!**

## Building the integration tool

Prerequisites:
- A rust toolchain (preferably installed with rustup)

Steps:
1. Clone this repo
2. cd into `srtb-integrate-speeds`
3. Run `cargo build` or `cargo run` from a command line
4. Profit

## License

This mod and the integration tool are licensed under the [MIT License](LICENSE).
