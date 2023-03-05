# Jobber

Command line tool for tracking work time.

## Contents

- [Jobber](#jobber)
  - [Contents](#contents)
  - [Purpose](#purpose)
  - [Usage](#usage)
    - [Enter Work Time](#enter-work-time)
      - [Start a New Job](#start-a-new-job)
      - [End an Open Job](#end-an-open-job)
      - [Add a New Job](#add-a-new-job)
      - [Back to Work](#back-to-work)
      - [Duration](#duration)
    - [Visualizing Entered Work Times](#visualizing-entered-work-times)
      - [List View](#list-view)
      - [Report View](#report-view)
      - [Filter Your View](#filter-your-view)
    - [Select Database](#select-database)
  - [Date, Time, Duration and Range Formats](#date-time-duration-and-range-formats)
    - [Date and/or Time](#date-andor-time)
    - [Durations](#durations)
    - [Ranges](#ranges)
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

## Purpose

I started *jobber* as a *Ruby* script in 2013 because in my opinion usual work time tracking tools often come with awful or overloaded UIs.
To be more convenient for people who can use a command shell, *jobber* aims to provide it's functionality by the command line.
After 10 years I still find it very useful but although the *Ruby* script is very handy, it is still full of bugs and just did the minimum job even if it promised much more complex functionality but wasn't able to fulfill it's promises.

In 2021 I started to learn *Rust* and so I decided to rewrite jobber in that language to get a proper working version that provides all functionality in a sustainable way.
After several approaches it turned out that *Rust* was a good decision because *Rust* is much more picky about edge cases and has a nice testing environment to prevent any hidden erroneous code which might lead to wrong accounting.

After several month of coding I now can present a first testable version which is not perfect (I still have some more ideas I want to implement) but yet seems more useful and secure than the original *Ruby* version.

## Usage

The idea of *jobber* is that you don't need a UI where you use your mouse or a smartphone touch screen to enter what you did into a form which seemed awful to me most of the time.

So in general the following information must be provided to track your work time:

- time when you start to work
- time when you finished your work
- a message about what you did
- some information to categorize your work (a client, a topic)

That's it.

### Enter Work Time

So the basic idea is to provide this in an easy to use command line like this:

```txt
jobber -s start_time -e end_time -m message -t tags
```

Of couse it makes sense to start a job and later finish it.
So you can start a job with `-s` and finish it later with calling jobber again with `-e`.
Also leaving a message often is easier when you know what you have done so you might provide a message when you finish the job - same with the tags.

Providing start and end time would be hard if you have to write done complete date and time every time so you can shorten it to what's necessary like `12:00` for today at noon or just no time to mean right now.

Let's get into an example.

#### Start a New Job

We use the start option `-s` to start a new job:

```txt
▶ jobber -s
Beginning new database file 'jobber.json'
Started new job:

  Start: Sat Mar 04 2023, 16:25
    End: - open -


Saved database into file 'jobber.json'
```

As you can see *jobber* tells you that it began with a new database in a file called `jobber.json` and has started a new job which end is still open.
It also assures you that changes were written into that file.

If you use a shell which provides color start time will be green and end time will be purple for better reading but in this README which is markdown sadly this can not be visualized.

We can check what we have done by using the list option `-l` (see also section *List View*):

```txt
▶ jobber -l 
Loaded database (1 entries) from file 'jobber.json'
    Pos: 1
  Start: Sat Mar 04 2023, 16:25
    End: - open -
  Hours: 0.25


Total: 1 job(s), 0.25 hours

Database unchanged.
```

*jobber* prints out a list of all stored jobs and as you can see there is this one open job we've just started and because some time already elapsed the job hours is given there with a quarter of an hour.
By default there is a time resolution of 15 minutes in which work times are calculated.
This resolution can be changed but for now we let it to the default.

#### End an Open Job

So let's end the job because let's assume we did something useful and want to finish by using the end option `-e`:

```txt
▶ jobber -e
Loaded database (1 entries) from file 'jobber.json'
You need to enter a message about what you did to finish the job.
Finish input with empty line (or Ctrl+C to cancel):
Did some nice work 

Modified job:

    Pos: 1
  Start: Sat Mar 04 2023, 16:25
    End: Sat Mar 04 2023, 16:34
  Hours: 0.15
Message: Did some nice work

Saved database into file 'jobber.json'

```

As you can see *jobber* detects that you haven't given a description about what you have done and so asks you for a message to enter.
I replied with `Did some nice work`.
The message could be multiline but for now we use a single line.

After I entered the message *jobber* reports that it modified the open job and writes it down as it is now stored in the database file.

So we successfully finished our first job.

#### Add a New Job

Now let's add another job we did this morning and forgot to enter then.
And this time we give all the data in the command line by using `-s` and `-e` with a time and the message option `-m` to give the message without being asked for:

```txt
▶ jobber -s 8:15 -e 10:45 -m "What I did this morning"    
Loaded database (1 entries) from file 'jobber.json'
Added new job:

  Start: Sat Mar 04 2023, 08:15
    End: Sat Mar 04 2023, 10:45
  Hours: 2.5
Message: What I did this morning

Saved database into file 'jobber.json'
```

As you can see this also worked like a charm.

#### Back to Work

You can replace the `-s` by `-b` to continue a job.
Then a new job will be created like with `-s` but message and tags of the last job will be taken automatically for the new one.

#### Duration

Instead of giving an end date and/or time with `-e` you can also user `-d` to give a duration of the job (see section
*Durations*).

### Visualizing Entered Work Times

#### List View

Let's take a look what we already did today:

```txt
▶ jobber -l                                           
Loaded database (2 entries) from file 'jobber.json'
    Pos: 1
  Start: Sat Mar 04 2023, 16:25
    End: Sat Mar 04 2023, 16:34
  Hours: 0.25
Message: Did some nice work

    Pos: 2
  Start: Sat Mar 04 2023, 08:15
    End: Sat Mar 04 2023, 10:45
  Hours: 2.5
Message: What I did this morning

Total: 2 job(s), 2.75 hours

Database unchanged.

```

You can see that jobber lists the two jobs we did.

#### Report View

Let's display this in a more fancy view with the report option `-r`:

```txt
▶ jobber -r
Loaded database (2 entries) from file 'jobber.json'
                               3/2023                               
day     sun     mon     tue     wed     thu     fri     sat    week
                                  -       -       -    2.75    2.75
  5       -       -       -       -       -       -       -       0
 12       -       -       -       -       -       -       -       0
 19       -       -       -       -       -       -       -       0
 26       -       -       -       -       -                       0
                                                 Mar 2023: 2.75 hrs.

Total: 2 job(s), 2.75 hours
Database unchanged.
```

Wow! What you get now is a monthly report of the jobs.

Let me explain how this is to read: In the first line of the report you see the month and the year `3/2023`.
After that a table follows in which the first column shows the day of month for each line (except the first which would always be `1`).
The next seven columns show the work time for each day.
Because I write this README at Saturday under `sat` you see that I worked `2.75` hours today.
In the last column the weekly work time is summed up and at the end of the table it says that we work the same amount in all of March and - as useless as it seems in our case - at the end it sums up all work time for all displayed jobs.

#### Filter Your View

You might use `-t` to set up some tags which will filter out every job not using these tags in all reports.

### Select Database

With `-f` you can set which database file shall be used by *jobber*:

```txt
▶ jobber -f ~/my_jobber.json` [...]
```

## Date, Time, Duration and Range Formats

### Date and/or Time

Date and time in one of the following formats:

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

**Notes:**

- Spaces within the time formats are not allowed and combined date and time formats can also be swapped to time and date.
- When date or time is missing current time will be used.
- If giving start and end together only one needs to define a date

### Durations

Duration in one of the following formats:

| Format     | Type               | Description                   | Example |
| :--------- | ------------------ | ----------------------------- | ------- |
| *H*`:`*M*  | standard           | hours and minutes             | `1:15`  |
| *h*`,`*fr* | with comma         | hours and fraction of an hour | `1,25`  |
| *h*`.`*fr* | with decimal point | hours and fraction of an hour | `1.25`  |

### Ranges

Time or positional range in one of the following formats:

| Format     | Description               | Example            |
| :--------- | ------------------------- | ------------------ |
| *f*`-`*t*  | from position to position | `3-5`              |
| *f*`-`     | since position            | `3-`               |
| *C*        | count (from end)          | `10`               |
| *s*`..`*u* | since time until time     | `1/31,15:00..1.2.` |
| *s*`..`    | since time                | `1/31,15:00..`     |
| *D*        | single day                | `1/31`             |

**Hints:**

- when using *since time until time* or *since time* format together with *decimal point date without year* remember that three points will be in the middle (e.g. `31.1...1.2.`)

## Errors

### I/O error

Reading or writing to file has failed.

### JSON error

Parsing *JSON* went wrong.

### There still is an open job

You have tried to start a new job but there currently is an open job which needs to be ended before you can add new jobs.

### There is no open job

You tried to end an open job but there is none.

### End of the job is before it's start

End and start time seems swapped in order.

### User cancel

You refused something after questioned.

### Can not use tags within same job because they have different configurations

You used two tags together which have different configurations.

### User needs to enter message

You need to enter a message but you did not.

### Unknown column name

You stated a column name that is unknown when exporting into *CSV* (only `pos`, `start`, `end`, `hours`, `message` and `tags` are available).

### Date/Time parse error

Parsing of a date and or time went wrong.
