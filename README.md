# Jobber

Command line tool for tracking work time.

## Contents

- [Jobber](#jobber)
  - [Contents](#contents)
  - [Purpose](#purpose)
  - [Installation](#installation)
  - [Usage](#usage)
    - [Entering Work Times](#entering-work-times)
      - [Starting a New Job](#starting-a-new-job)
      - [Ending an Open Job](#ending-an-open-job)
      - [Adding a New Job](#adding-a-new-job)
      - [Back to Work](#back-to-work)
      - [Duration](#duration)
      - [Tagging your Jobs](#tagging-your-jobs)
    - [Editing Jobs](#editing-jobs)
    - [Deleting Jobs](#deleting-jobs)
    - [Dry Run](#dry-run)
    - [Visualizing Entered Jobs](#visualizing-entered-jobs)
      - [Listing Jobs](#listing-jobs)
      - [Reporting by Work Days](#reporting-by-work-days)
      - [Filter Your View](#filter-your-view)
    - [Select Database](#select-database)
  - [Date, Time, Duration and Range Formats](#date-time-duration-and-range-formats)
    - [Date and/or Time](#date-andor-time)
    - [Durations](#durations)
    - [Ranges](#ranges)
  - [Import and Export](#import-and-export)
    - [Legacy CSV Import](#legacy-csv-import)
    - [CSV Export](#csv-export)
  - [Warnings](#warnings)
    - [The job you want to add overlaps existing one(s)](#the-job-you-want-to-add-overlaps-existing-ones)
    - [You have used some tags which are unknown](#you-have-used-some-tags-which-are-unknown)
  - [Errors](#errors)
    - [I/O error](#io-error)
    - [JSON error](#json-error)
    - [There still is an open job](#there-still-is-an-open-job)
    - [There is no open job](#there-is-no-open-job)
    - [End of the job is before it's start](#end-of-the-job-is-before-its-start)
    - [User cancel](#user-cancel)
    - [Can not use tags within same job because they have different configurations](#can-not-use-tags-within-same-job-because-they-have-different-configurations)
    - [User needs to enter message](#user-needs-to-enter-message)
    - [Unknown column name](#unknown-column-name)
    - [Date/Time parse error](#datetime-parse-error)
  - [Configuration](#configuration)
    - [Location of Database](#location-of-database)
    - [Database Internal Configuration](#database-internal-configuration)
      - [Setup Base Configuration](#setup-base-configuration)
        - [Work Time Resolution](#work-time-resolution)
        - [Hourly Payment Rate](#hourly-payment-rate)
        - [Maximum Hours Per Day](#maximum-hours-per-day)
      - [Setup Configuration for Specific Tags](#setup-configuration-for-specific-tags)
      - [Show Configuration](#show-configuration)

## Purpose

I started *jobber* as a *Ruby* script in 2013 because in my opinion usual work time tracking tools often come with awful or overloaded UIs.
To be more convenient for people who can use a shell, *jobber* aims to provide it's functionality by the command line.
After 10 years I still find it very useful but although the *Ruby* script is very handy it is still full of bugs and just did the minimum job even if it promised much more complex functionality but wasn't able to fulfill its promises.

In 2021 I started to learn *Rust* and so I decided to rewrite *jobber* in that language to get a proper working version that provides all functionality in a sustainable way.
After several approaches it turned out that *Rust* was a good choice because it is much more picky about edge cases and has a nice testing environment to prevent any hidden erroneous code which might lead to wrong accounting.

After several month of coding I now can present a first testable version which is not perfect (I still have some more ideas I want to implement) but yet seems more useful and secure than the original *Ruby* version.

## Installation

To install *jobber* you need to install *Rust* via *rustup* (see <https://rustup.rs>) and then use the following line:

```txt
▶ cargo install jobber
```

Now you can use jobber in your command line.

## Usage

The idea of *jobber* is that you don't need an UI where you use your mouse or a smartphone touch screen to enter what you did into a form which seemed awful to me most of the time.

So in general just the following information must be provided to track your work times:

- time when you start your work
- time when you finished your work
- a message about what you did
- some information to categorize your work (a client, a topic)

That's it.

### Entering Work Times

So the basic idea is to provide this in an easy to use command line like this:

```txt
jobber -s start_time -e end_time -m message -t tags
```

Of course it makes sense to start a job and later finish it.
So you can start a job with `-s` and finish it later by calling jobber again with `-e`.
Also leaving a message often is easier when you know what you have done so you might provide a message when you finish the job.
Same with tags.

Providing start and end time would be hard if you have to write complete date and time every time.
So you can shorten it to what's necessary like `12:00` for today at noon or just no time to mean right now.

Let's get into an example.

#### Starting a New Job

We use the start option `-s` to start a new job:

```txt
▶ jobber -s
Beginning new database file '/home/pat/jobber.json'
Started new job:

  Start: Sat Mar 04 2023, 16:25
    End: - open -


Saved database into file '/home/pat/jobber.json'
```

As you can see *jobber* tells you that it began with a new database in a file called `jobber.json` in my home directory and has started a new job which end is still left open.
It also assures you that changes were written into that file.

If you use a shell which provides color, start time will be green and end time will be purple for better reading but in this README which is written in *Markdown* format sadly this can not be visualized.

We can check what we have done by using the list option `-l` (see also later in section *List View*):

```txt
▶ jobber -l 
Loaded database (1 entries) from file '/home/pat/jobber.json'
    Pos: 1
  Start: Sat Mar 04 2023, 16:25
    End: - open -
  Hours: 0.25


Total: 1 job(s), 0.25 hours

Database unchanged.
```

*Jobber* prints out a list of all stored jobs and as you can see there is this one open job we've just started and because some time already elapsed the job hours is given there with a quarter of an hour.
By default there is a time resolution of 15 minutes in which work times are calculated.
This resolution can be changed (see section *Configuration* below) but for now we let it to be the default.

#### Ending an Open Job

So let's end the job because let's assume we did something useful and want to finish by using the end option `-e`:

```txt
▶ jobber -e
Loaded database (1 entries) from file '/home/pat/jobber.json'
You need to enter a message about what you did to finish the job.
Finish input with empty line (or Ctrl+C to cancel):
Did some nice work

Modified job:

    Pos: 1
  Start: Sat Mar 04 2023, 16:25
    End: Sat Mar 04 2023, 16:34
  Hours: 0.15
Message: Did some nice work

Saved database into file '/home/pat/jobber.json'
```

As you can see *jobber* detects that we have not given any description about what we have done and so it asks us for a message to enter.
We replied with `Did some nice work`.
The message could be multiline but for now we use only a single line.

After we entered the message *jobber* reports that it modified the open job and then writes it down as it is stored now within the database.

So we successfully finished our first job.

#### Adding a New Job

Now let us add another job we did this morning and forgot to enter then.
And this time we give all the data in the command line by using `-s` and `-e` with a time and the message option `-m` to give the message without getting asked for it:

```txt
▶ jobber -s 8:15 -e 10:45 -m "What I did this morning"    
Loaded database (1 entries) from file '/home/pat/jobber.json'
Added new job:

  Start: Sat Mar 04 2023, 08:15
    End: Sat Mar 04 2023, 10:45
  Hours: 2.5
Message: What I did this morning

Saved database into file '/home/pat/jobber.json'
```

As you can see this also worked like a charm.

#### Back to Work

You can replace the `-s` by `-b` to continue a job.
Then a new job will be created like with `-s` but message and tags of the last job will be taken automatically for the new one.
This is useful if you make a break and continue your work afterwards.

#### Duration

Instead of giving an end date and/or time with `-e` you can also user `-d` to give a duration of the job (see section
*Durations* for duration formats).

#### Tagging your Jobs

If you want to categorize your jobs (e.g. to differ meetings from programming jobs) or if you have multiple clients you work for you can use tags to set this up.

Just add option `-t` to give a list of tags when you create a job:

```txt
▶ jobber -s -t meeting -m "meeting about new design" -d 2:00
Loaded database (2 entries) from file '/home/pat/jobber.json'
There is one warning you have to omit:

WARNING 1) You have used some tags ( meeting ) which are unknown so far. Continue if you want to create them.
Do you still want to add this job? (y/N)
y
Added new job:

  Start: Sun Mar 05 2023, 21:24
    End: Sun Mar 05 2023, 23:24
  Hours: 2
   Tags:  meeting  
Message: meeting about new design

Saved database into file '/home/pat/jobber.json'
```

Here *jobber* asks us if we want to add the unknown tag `meeting` and we answered yes by entering `y`.

Tags are colored differently after they were added so that you can easily differentiate between them (you can not see this here in this *Markdown* file).

You can add multiple tags by listing them separated by comma (e.g. `meeting,design`).

You also can use tags to differ between clients and give every client a different configuration (see section *Configuration*).

### Editing Jobs

Jobs can be edited by using `--edit <POS>` then add some `-s`, `-e`, `-d`, `-m` or `-t` to change single properties.
The only property which can be forced to change to empty is `-t`.
By giving no tags to `-t` tags will be deleted when editing.

```txt
▶ jobber --edit 2 -m "What I did early this morning"
Loaded database (3 entries) from file 'jobber.json'
Modified job:

    Pos: 2
  Start: Sat Mar 04 2023, 08:15
    End: Sat Mar 04 2023, 10:45
  Hours: 2.5
Message: What I did early this morning

Saved database into file 'jobber.json'
```

### Deleting Jobs

You can delete jobs by ranges (like you can use in `-r` or `-l`) and you will get asked before deletion is done.
Deleted jobs will not be removed from the database but marked internally with the date and time of deletion (this is for further use).

Any deleted job won't appear in any report, export or listing.

```txt
▶ jobber --delete 3-6  
Loaded database (138 entries) from file '/home/pat/git/ts.officestuff/jobber.json'
There ist one warning you have to omit:

WARNING 1) You are about to delete job(s) at the following position(s): 3-6
Do you still want to add this job? (y/N)
y
Deleting job(s) at position(s): 3-6
Saved database into file 'jobber.json'
```

### Dry Run

If you are experiencing with *jobber* and you want to be safe that your database won't be corrupted by any wrong input, you may use `-D` to process a so-called *Dry Run*.

In a *Dry Run* the everything works as usual but the database won't be saved into the filesystem.

```txt
git/private/jobber  master ✗                                                                                                                                                            5m ⚑  
▶ jobber -D --delete 3-6
Loaded database (138 entries) from file '/home/pat/git/ts.officestuff/jobber.json'
There ist one warning you have to omit:

WARNING 1) You are about to delete job(s) at the following position(s): 3-6
Do you still want to add this job? (y/N)
y
Deleting job(s) at position(s): 3-6
DRY RUN: Changes were NOT saved into database file '/home/pat/git/ts.officestuff/jobber.json'!
```

This example is the same as above (in section *Deleting Jobs*) but the last message tells you, that `Changes were NOT saved`.

### Visualizing Entered Jobs

#### Listing Jobs

Let's take a look what we already did today:

```txt
▶ jobber -l                                           
Loaded database (2 entries) from file '/home/pat/jobber.json'
    Pos: 1
  Start: Sat Mar 04 2023, 16:25
    End: Sat Mar 04 2023, 16:34
  Hours: 0.25
Message: Did some nice work

    Pos: 2
  Start: Sat Mar 04 2023, 08:15
    End: Sat Mar 04 2023, 10:45
  Hours: 2.5
Message: What I did early this morning

    Pos: 3
  Start: Sun Mar 05 2023, 21:24
    End: Sun Mar 05 2023, 23:24
  Hours: 2
   Tags:  meeting  
Message: meeting about new design

Total: 3 job(s), 4.75 hours

Database unchanged.

```

You can see that jobber lists the three jobs we did.

#### Reporting by Work Days

Let's display this in a more fancy view with the report option `-r`:

```txt
▶ jobber -r
Loaded database (2 entries) from file '/home/pat/jobber.json'
                               3/2023                               
Day     Sun     Mon     Tue     Wed     Thu     Fri     Sat    Week
                                  -       -       -       3       3
  5       2       -       -       -       -       -       -       2
 12       -       -       -       -       -       -       -       0
 19       -       -       -       -       -       -       -       0
 26       -       -       -       -       -                       0
                                                    Mar 2023: 5 hrs.

Total: 3 job(s), 5 hours
Database unchanged.
```

Wow! What you get now is a monthly report of the jobs.

Let me explain how this is to read: In the first line of the report you see the month and the year `3/2023`.

After that a table follows in which the first column shows the day of month for each line (except the first which would always be `1`).

The next seven columns show the work time for each day.
Because I write this README at Saturday under `sat` you see that I worked `3` hours at Saturday and `2` at Sunday.

In the last column the weekly work time is summed up and at the end of the table it says that we work the same amount in all of March and - as useless as it seems in our case - at the end it sums up all work time for all displayed jobs.

#### Filter Your View

You can add a time range behind `-r` (see section *Ranges* below for formats) or use `-t` to filter jobs by time or tags.

### Select Database

Usually jobber uses the database listed in the configuration file (see section *Configuration* below).
With `-f` you can overwrite which database file shall be used instead:

```txt
▶ jobber -f ~/my_jobber.json` [...]
```

## Date, Time, Duration and Range Formats

### Date and/or Time

Date and time have to be entered in one of the following formats:

| Format                      | Type        | Description                     | Example          |
| :-------------------------- | ----------- | ------------------------------- | ---------------- |
| *m*`/`*d*`/`*y*`,`*H*`:`*M* | month first | date and time                   | `1/31/2023,1:01` |
| *m*`/`*d*`/`*y*             | month first | date                            | `1/31/2023`      |
| *m*`/`*d*`,`*H*`:`*M*       | month first | date without year and with time | `1/31,1:01`      |
| *m*`/`*d*                   | month first | date without year               | `1/31`           |
| *d*`.`*m*`.`*y*`,`*H*`:`*M* | day first   | date and time                   | `31.1.2023,1:01` |
| *d*`.`*m*`.`*y*             | day first   | date                            | `31.1.2023`      |
| *d*`.`*m*`.,`*H*`:`*M*      | day first   | date without year and with time | `31.1.,1:01`     |
| *d*`.`*m.*                  | day first   | date without year               | `31.1.`          |
| *y*`-`*m*`-`*d*`,`*H*`:`*M* | year first  | date and time                   | `2023-1-31,1:01` |
| *y*`-`*m*`-`*d*             | year first  | date                            | `2023-1-31`      |
| *H*`:`*M*                   | -           | time                            | `1:01`           |

Spaces within the time formats are not allowed and combined date and time formats can also be swapped to time and then date.

When date or time is missing current time will be used.

If giving start and end together only one needs to define a date.

### Durations

Durations have to be entered in one of the following formats:

| Format     | Type               | Description                   | Example |
| :--------- | ------------------ | ----------------------------- | ------- |
| *H*`:`*M*  | standard           | hours and minutes             | `1:15`  |
| *h*`,`*fr* | with comma         | hours and fraction of an hour | `1,25`  |
| *h*`.`*fr* | with decimal point | hours and fraction of an hour | `1.25`  |

### Ranges

Time or positional range have to be entered in one of the following formats:

| Format     | Description               | Example            |
| :--------- | ------------------------- | ------------------ |
| *f*`-`*t*  | from position to position | `3-5`              |
| *f*`-`     | since position            | `3-`               |
| *p*        | single position           | `3`                |
| `~`*C*     | count (from end)          | `10`               |
| *s*`..`*u* | since time until time     | `1/31,15:00..1.2.` |
| *s*`..`    | since time                | `1/31,15:00..`     |
| *D*        | single day                | `1/31`             |

When using *since time until time* or *since time* format together with *decimal point date without year* remember that three points will be in the middle (e.g. `31.1...1.2.`).

## Import and Export

### Legacy CSV Import

To import an old database format from the legacy *Ruby* version of *jobber* you can use `--legacy-import`:

```txt
▶ jobber --legacy-import ~/my_old_jobber.dat 
Loaded database (2 entries) from file '/home/pat/jobber.json'
Imported 125 jobs added new tags  consult ,  meeting ,  my_client ,  training .
Saved database into file '/home/pat/jobber.json'
```

*Jobber* shows that it has successfully imported `125 jobs` and that four new tags came with this import (again: tag names would be colored but not here in the *Markdown* text).

### CSV Export

By using the option `-E` you can export the database or parts of it into a CSV file for example to create your an invoice from it:

```txt
▶ jobber -E
Loaded database (3 entries) from file 'git/private/jobber/jobber.json'
"tags","start","hours","message"
"","03/04/2023 08:15",2.5,"What I did early this morning"
"","03/04/2023 16:25",0.5,"Did some nice work"
"meeting","03/05/2023 21:24",2,"meeting about new design"
Database unchanged.
```

As you can see the default output is a CSV file including the following columns: `tags`, `start`, `hours` and `message`.

Items in export are automatically sorted by start date and time for your convenience.

To write output into a file use the pipe feature of your shell (e.g. `jobber -E > out.csv`)

To change the columns which are exported you can use option `--csv` (possible values are: `pos`, `tags`, `start`, `end`, `hours`, `pay` and `message`):

```txt
▶ jobber -E --csv pos,start,end
Loaded database (3 entries) from file 'git/private/jobber/jobber.json'
"pos","start","end"
2,"03/04/2023 08:15","03/04/2023 10:45"
1,"03/04/2023 16:25","03/04/2023 16:34"
3,"03/05/2023 21:24","03/05/2023 23:24"
Database unchanged.
```

In this example we just exported `pos`,`start` and `hours` of each job.

You may also specify a range with option `-E` like when you do a report:

```txt
▶ jobber -E 3/4,0:00..12:00
Loaded database (3 entries) from file '/home/pat/jobber.json'
"tags","start","hours","message"
"","03/04/2023 08:15",2.5,"What I did early this morning"
Database unchanged.
```

As you can see now only one of both jobs has been exported.

## Warnings

*jobber* does several plausibility checks of your commands.
Whenever something seems unintended you will get a warning and will be asked if you still want to continue.

### The job you want to add overlaps existing one(s)

You are about to add a job that intersects another one in time.

```txt
▶ jobber -s 3/4,9:00 -e 10:00
Loaded database (3 entries) from file '/home/pat/jobber.json'
There is one warning you have to omit:

WARNING 1) The job you want to add overlaps existing one(s):

Job you want to add:

  Start: Sat Mar 04 2023, 09:00
    End: Sat Mar 04 2023, 10:00
  Hours: 1


Existing overlapping jobs:

    Pos: 2
  Start: Sat Mar 04 2023, 08:15
    End: Sat Mar 04 2023, 10:45
  Hours: 2.5
Message: What I did early this morning

Total: 1 job(s), 2.5 hours

Do you still want to add this job? (y/N)
```

*Jobber* lists the job you are about to add and then the one that intersects.

If you start a job (without giving an end) and start time is before any existing job it will be assumed that ending might be now and so you might get a warning for this.
Then check your input and if this was intentional answer `y` to continue or `n` to cancel.

### You have used some tags which are unknown

With option `-t` you have given some tag names which are currently unknown and *jobber* then asks you if this was a mistake or if you want to add the new tag:

```txt
▶ jobber -s -t consulting 
Loaded database (3 entries) from file '/home/pat/jobber.json'
There is one warning you have to omit:

WARNING 1) You have used some tags ( consulting ) which are unknown so far. Continue if you want to create them.
Do you still want to add this job? (y/N)
```

In this case the tag `consulting` is unknown.

To list which tags are already known you can use the option `-T` (see section *Tagging*).

## Errors

### I/O error

Reading or writing to file has failed.

### JSON error

Parsing *JSON* went wrong the database file may be corrupted.

### There still is an open job

You have tried to start a new job but there currently is an open job which needs to be ended before you can add new jobs.

### There is no open job

You tried to end an open job but there is none.

### End of the job is before it's start

End and start time seem to be swapped in order.

### User cancel

You refused something after questioned.

### Can not use tags within same job because they have different configurations

You used two tags together which have different configurations which is not permitted.

### User needs to enter message

You need to enter a message but you did not.

### Unknown column name

You stated a column name that is unknown while exporting jobs into a *CSV* file (only `pos`, `start`, `end`, `hours`, `message` and `tags` are available).

### Date/Time parse error

Parsing of a date and or time went wrong.

## Configuration

### Location of Database

At the first start *jobber* creates a configuration file (usually within your home directory at `.config/jobber/config.toml`)

This file has currently only one entry which is:

```txt
database = '/home/pat/jobber.json'
```

Change the path of the database if you like to have your database elsewhere.

### Database Internal Configuration

There are some settings within the *jobber* database you may want to change:

- work time resolution
- your hourly payment rate
- maximum hours per day

In *jobber* there is a base configuration but you also can attach configurations to tags to have different configurations for different clients by adding a tag list with `-t` when you change the settings.

#### Setup Base Configuration

##### Work Time Resolution

With `-R` you can set the work time resolution.
The default is `0.25` which means a quarter of an hour.
So if you add for example a job with a duration of 16 minutes this will be rounded up to 30 minutes.

For example you might change the base resolution to half an hour with:

```txt
▶ jobber -R 0.5       
Beginning new database file '/home/pat/jobber.json'
Changed the following default configuration values:

Resolution: 0.5 hours

Saved database into file '/home/pat/jobber.json'
```

##### Hourly Payment Rate

To change your hourly payment rate use `-P`.
There is no default so if you do not set this no rates will be displayed.
If you do so *jobber* lists costs when listing.

```txt
▶ jobber -P 100
Loaded database (0 entries) from file '/home/pat/jobber.json'
Changed the following default configuration values:

Payment per hour: 100

Saved database into file '/home/pat/jobber.json'
```

##### Maximum Hours Per Day

If you set this value with `-H` days which's work time exceeds this value will be marked yellow in the report.
When listing jobs that exceed this value will be marked yellow.

So if you want to change the maximum hours for a day to 8 use:

```txt
▶ jobber -H 8  
Loaded database (0 entries) from file '/home/pat/jobber.json'
Changed the following default configuration values:

Maximum work time: 8 hours

Saved database into file '/home/pat/jobber.json'
```

#### Setup Configuration for Specific Tags

If you have several clients and each one has for example different payment rates you can add the tag option `-t` when you set the configuration.

#### Show Configuration

To show your configuration(s) use the option `-C`:

```txt
▶ jobber -C  
Loaded database (3 entries) from file '/home/pat/jobber.json'
Base Configuration:

Resolution: 0.5 hours
Payment per hour: 100
Maximum work time: 8 hours

Database unchanged.
```
