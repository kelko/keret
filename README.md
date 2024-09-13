keret 
===================

keret ("kelkos embedded rust example: tracker") is pronounced like the english word "carrot".

As the full name indicates it is an example I made to try out and familiarize myself with embedded rust programming. It uses a [micro:bit v2 microcontroller](http://microbit.org/new-microbit/) and the code is heavily influenced by / based upon the [rust embedded discovery book](https://docs.rust-embedded.org/discovery/microbit/).


Scenario
-------

The micro:bit controller is used as physical control for a activity timer. The user can start timing an activiy by pressing the "B" button. Hitting the "B" a second time stops the activity and reports the duration via serial port.

A started activity (as well as potential error states) can be reset by pressing the "A" button.

On a (USB-)connected device (e.g. a RasPi) the report is read from the serial port and extended by current timestamp before it's send via HTTP POST to a service.

Somewhere (e.g. inside a Kubernetes cluster or locally on some host) there is the service running, receiving the full report (timestamp and duration) and storing it onto disc. This list can be read from the service.

Goals
---------

This example tries to show how to program a microcontroller via Rust. It uses the display, the buttons, the internal timer and the serial port of the micro:bit board. And while doing to I tried to built it in a structured way with "typical" error handling as I would do it for CLIs / services on a fully-fledged OS-driven device.

To see how basically the same way of programming is possible on such a restricted environment (e.g. no heap -> no alloc) as is on fully-fledged devices or in the cloud the two other parts where also constructed. Each operates on a different level of hardware abstraction, but each code reads quite similar.


Components
--------------

- **keret-controller**: The code running on the micro:bit microcontroller
- **keret-adapter**: The CLI running on the connected device, reading the reports from the serial port and forwarding to the service
- **keret-service**: The REST API service receiving and storing the activity reports

TODO
-----

### time handing ("now()")

this is rudimentary at best and rather broken, as it does not yet handle timer wraps currently. Waiting for either microbit-v2 crate to switch to embassy or otherwise understand which interrupt is caused by the SYST clock.

But this does not hinder the usability of the example to showcase the original goal.
