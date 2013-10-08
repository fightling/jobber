Purpose
======

Command line tool for tracking work time.

Installation
============

You need ruby 1.8.7 to run this script.

Just copy the script into your bin and do a chmod to make it executable

    chmod u+x jobber.rb

Usage
=====

Start time tracking
-------------------

Start tracking now:

    $ jobber -s
    Starting new job:
        Pos: 1
      Start: Tue Oct 08 2013, 07:51

Start tracking at specific time in past:
    
    $ jobber -s 12:00
    Starting new job:
        Pos: 1
      Start: Tue Oct 08 2013, 12:00

*The absolute time is interpreted as like it is within +/-12 hours. 
 So if it's currently 8:00h, 21:00 will be yesterday. 17:00 would be in future.*

Start tracking at absolute date and time:
    
    $ jobber -s 10/30/2013,12:00

Equivalents are:

    $ jobber -s 30.10.2013,12:00
    $ jobber -s 12:00,10/30/2013
    $ jobber -s 12:00,30.10.2013

Start tracking at relative time:
    
    $ jobber -s 5h-

Giving a number followed by a "h" or "m" and a "+" or "-" the resulting time will be calculated by a distance in hours or minutes from now.

End time tracking
-----------------

End tracking now:
    
    $ jobber -e

End tracking at specific time in past:
    
    $ jobber -e 14:00

End tracking at absolute date and time:
    
    $ jobber -e 10/30/2013,14:00

End tracking at relative time:
    
    $ jobber -e 1m+

List jobs
---------
List all known jobs:
    
    $ jobber -l

Monthly report:
    
    $ jobber -r
    jobber - job time tracker
                             4/2013                         
         sun     mon     tue     wed     thu     fri     sat
               11.75       -   12.75       -       -       -
           -       -     4.5     6.5       -     7.5       -
           -       -       -       -       -       -       -
           -       -       -       -       -       -       -
           -       -       -
                   Monthly total: 43.0 hours                

    Total over all: 43.0 hours

