# Headifier


https://github.com/serranoio/headifier/assets/75829857/e1426da1-0945-44b4-a897-56488f60630d


## Welcome to Headifier!

* Add headers to all of your projects using a neat terminal UI

## Install

1. from the command line, run
    `cargo install headifier`
2. Enter project directory where you want to add headers
3. Run the binary! 
    `headifier`
4. Follow on screen instructions


## Notes
* Currently the only way to undo is to remove the changes via git  

## RoadMap
 - The release cycle happens once per month, and will happen on the first per month.
    - Next small update will be ready on December 1st.
    - A breaking bug will be fixed ASAP
    - convenience bugs will be released every now and then

### Features
1. Replace option that will replace all current headers with the one you want!
2. Creating header can apply to any file - it'll append // to every line for .ts,.js,.rs, and for css it will be /* */, html <!-- --> 

### Optimizations
1. buttons

#### Bugs
1. remove spaces from gitignore
2. .tsx should only change .tsx files and .ts should only change .ts
