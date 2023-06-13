# plate-tool (working title)

A web-based tool for creating assays for your favorite (acoustic) liquid handler.

## Table of Contents
- [Usage](#Usage)
- [Installation](#Installation)

## Usage

### Adding plates
When you open plate tool for the first time,
you'll be greeted by a message informing you that no plates are selected.
     
To add a new plate, click the "New Plate" button:
     
 Once you've added at least one source plate and one destination plate,
 click one of each to select them.
 The right-most pane will now display these plates.
 
 ### Modifying and deleting plates
 Suppose you erroneously created a plate, or misspelled its name.
 Double click on that plate in the list (top-left pane) and a new modal will open.
 Here you can rename a plate or delete it.
 
 ### Adding a transfer
 Now that you have two plates selected,
 it's time to add a transfer.
 We can see all of the properties of our transfer in the bottom-left pane.
 You should first name your transfer (this name is only used for your reference, and is not passed to the liquid handler).
 You can enter your source and destination regions in their respective fields;
 the accepted format should be familiar—capital letters for the row and arabic numerals for the column.
 
 However, it is much easier to click-and-drag the desired region.
 If we click and hold on a well (see right pane), that specifies our start well.
 Then, we can drag and subsequently release on our desired end well.
 
 Our selected wells will be highlighted in light blue for our source plate and light red for our destination plate.
 You might also notice that some wells are hatched:
 this indicates wells that will be used in the transfer.
 Not all selected wells will necessarily be hatched,
 depending on the transfer type and interleave settings.
 
 When all of the settings are to your liking, click the "Save" button.
 Note that it now appears in the "Transfers" section of the list pane.

 ### Modifying and deleting transfers
 If you already saved a transfer and would like to change it,
 click on its entry in the list.
 Now change the properties of the transfer as you did during initial creation.
 When finished, click the "Save" button to commit these changes.
 
 If you no longer need a transfer, select it as above and then click the "Delete" button.
 
 ### Importing and Exporting
 
 Exporting the transfers we have created to a CSV format is the primary (if not sole) usage of Plate Tool.
 To do so, first note the "File" tab at the top-left of the screen (above the list pane).
 Mouse over this tab, and a few more options will be revealed.
 We want to export: mouse over export and select "Export as CSV".
 You will be reminded that this is a one-way export (see JSON export/import below),
 and then prompted by your browser to select a location for your file.
 
 Currently, it is not possible to import from nor export to a format produced by other similar software.
 However, you might reasonably want to save a copy of your work
 either as a backup or to share.
 Mouse over the "File" tab, then "Export" as above, then alternatively select "Export as JSON". 
 Your browser will then prompt you to pick a suitable location to save your work as a file.
 (See note 1 below)
 
 If we want to import one such file, mouse over the "File" tab as before
 and select "Import".
 This opens a modal where you are prompted to upload (see note 2)
 your file; it will then be processed and loaded.
 Keep in mind that this will overwrite any work you currently have open,
 so you may wish to export first (see above).
 
 _Note 1_: JSON files are plaintext!
 By default there is little whitespace (this makes comprehending them a challenge)
 but if we pass it through a "JSON Beautifier" (enter this into your search engine of choice)
 it immediately becomes more readable.
 It is encouraged (although by no means necessary) to take a look at your export;
 you will see that the representation here very closely mirrors the representation presented
 in Plate Tool.
 
 _Note 2_: Use of the word "upload" might imply that your data is leaving your computer.
 It does not.
 You are welcome to verify (use your browser's developer tools, it should have a network tab)
 that this application does not "phone home".
 Your data is stored locally (unless you choose to export it and distribute it yourself).

## Installation

Plate tool is hosted [here](https://ilia.moe/cool-stuff/plate-tool/) for your convenience.
However, you're absolutely welcome to host your own instance (even locally).
Here's how:
(_Note:_ If you run Windows you're probably best off doing the following in WSL2)

1. Make sure you have a working Rust toolchain
    1. Installing `rustup` is the easiest way to do this. See [their website](https://rustup.rs/),
  or consult documentation provided by your distribution.
    2. Add the `wasm32-unknown-unknown` target with `rustup target add wasm32-unknown-unknown`
    - Generally, Rust's tools give descriptive errors and will help you solve problems yourself.
2. Install [trunk](https://trunkrs.dev/)
    - Run `cargo install --locked trunk`
3. Clone this repository using git
4. Enter the project directory and run `trunk serve`
