# Dynamic Track Speed

Scroll Velocities in SRXD. Finally!

## How to use

The mod is made to be as simple to use as possible. You will need:
- A text editor
- The chart (and the chart's file name)

To get started adding speed triggers, open your customs directory and create a new file with the same name as the SRTB file you want to add speed triggers to, but end it with the `.speeds` extension.

For example if you want to add triggers to `NULCTRL.srtb`, create a new file called `NULCTRL.speeds` **right next to `NULCTRL.srtb`**. They must be in the same directory.

> [!TIP]
> 
> If you are charting multiple difficulties, you can make a speeds file specific to one difficulty by appending its name in all caps to the filename.
> 
> Example: If you are charting NULCTRL in easy and expert and want to isolate the speed triggers, create 2 `NULCTRL_EASY.speeds` and `NULCTRL_EXPERT.speeds` files.
> 
> If you have other difficulties that don't have their own separate speeds file, they will use the global speeds file with no difficulty append.

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

# v1.4.0 addition: Repeat blocks
# Repeat blocks will repeat whatever is inside the defined block X times
# and add Y to the trigger time for each iteration.
# You cannot put repeat blocks inside other repeat blocks.
# Syntax:
Repeat X interval Y
<triggers>
EndRepeat

# Example:
Repeat 3 interval 1.0
30 0.5 true
30.5 1 false
EndRepeat

# This will be interpreted as:
# 30 0.5 true
# 30.5 1 false
# 31 0.5 true
# 31.5 1 false
# 32 0.5 true
# 32.5 1 false
```

Once the file is saved, go back to the chart listing and go to your chart. If you are already on your chart, go to another and come back, or click the Change Difficulty button if you are already in the editor. Every time you reload the chart preview, the mod will reload speeds, making quick editing convenient.

> [!IMPORTANT]
> 
> A little note on loading priority:
> 
> The mod will look for the following data and load whichever one it finds first
> 1. A dedicated speeds file for a specific difficulty
> 2. A dedicated speeds file for the whole chart
> 3. Dedicated integrated speeds for a specific difficulty
> 4. Legacy dedicated integrated speeds for the whole chart
> 
> If the mod finds none of these, no speed triggers will be loaded.

## Integration tool

Due to SpinShare uploading limitations, you can integrate the speed triggers directly in your SRTB by using a convenient little tool you can find on its [separate repository](https://github.com/Raoul1808/srtb-integration-program).

The tool is rather self-explanatory. You can integrate your speeds file in your chart or extract an existing chart's speeds file in case you want to analyze the speed triggers.

The tool comes in 2 flavours: a console version and a fresh new GUI version. If you encounter some issues with the GUI version, you can use the console version instead. Both tools work the same.

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

## License

This mod is licensed under the [MIT License](LICENSE).
