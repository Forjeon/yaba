# yaba

<img src="https://github.com/Forjeon/yaba/blob/main/yaba/webpages/favicon.svg" alt="yaba logo" width="100"/>

**Yet Another Budgeting App**

![Rust 1.83.0 badge](https://img.shields.io/badge/Rust-1.83.0-E33B26)
![Rocket 0.5.1 badge](https://img.shields.io/badge/Rocket-0.5.1-D33847)
![Diesel 2.2.5 badge](https://img.shields.io/badge/Diesel-2.2.5-FF2626)
![MySQL badge](https://img.shields.io/badge/MySQL-00618A)
![HTML badge](https://img.shields.io/badge/HTML-E54D26)
![CSS badge](https://img.shields.io/badge/CSS-663398)
![JavaScript badge](https://img.shields.io/badge/JavaScript-E5D24C)
![jQuery 3.7.1 badge](https://img.shields.io/badge/jQuery-3.7.1-0968AC)

Yaba is a minimalist personal budget and finance tracking webapp. Originally created as an undergrad project, yaba has since been improved upon for daily use in its intended function. Yaba aims to reduce feature bloat and ensure security of financial data through its privately hosted design.

---

[Getting Started](#getting-started)  
[Prerequisites and Dependencies](#prerequisites-and-dependencies)  
[Installation](#installation)  
[Configuration](#configuration)  

[Usage](#usage)  
[Features](#features)  
[API Guide](#api-guide)  

[Contribution](#contribution)  
[Prerequisites](#prerequisites)  
[Building](#building)  

[License](#license)  

---

## Getting Started

The yaba backend (Rust + Rocket) is hosted on a device of your choice and connected to a MySQL database using the Diesel migrations. At least one user profile must be created in the linked db. Then, while the yaba backend server is running, any web-capable device can access the yaba webapp to log and view budget/finance tracking data.

### Prerequisites and Dependencies

The yaba frontend is written in modern HTML, CSS, and JavaScript (plus jQuery). Thus, the frontend should be compatible with most any modern browser (tested and designed on qutebrowser and Vivaldi/Chrome).

Currently, the yaba backend server is only available as a [64-bit Linux binary LINK TODO](), and has only been tested on Ubuntu 22+. Feel free to clone and build binaries for other platforms as you see fit. Yaba also requires an active [MySQL server LINK TODO]() instance to host the webapp db. [Nginx LINK TODO]() is recommended as the reverse proxy for securing and configuring port forwarding of the yaba server.

### Installation

TODO

### Configuration

TODO


## Usage

TODO

### Features

TODO

### API Guide

TODO


## Contribution

At this time, yaba (Rust) is deprecated in favor of [yaba.NET LINK TODO](). You may clone, fork, and otherwise make use of this project and repository to your liking; pull requests and other changes to this repository are not being accepted at this time.

### Prerequisites

The yaba backend uses [Rust](https://www.rust-lang.org/tools/install) 1.83.0 with the [Rocket LINK TODO]() 0.5.1 and [Diesel LINK TODO]() 2.2.5 libraries, and requires an active [MySQL server LINK TODO]() instance to host the webapp db. [Nginx LINK TODO]() is also recommended as the reverse proxy to protect and configure port forwarding for the yaba server.

### Building

The yaba backend server is built using cargo 1.83.0.


## License

Copyright 2025 Jonathan Forsgren

This project is licensed under the [MIT license](https://github.com/Forjeon/yaba/blob/main/LICENSE).



STYLE GUIDE SUGGESTIONS: active voice, second person "you" to address reader, consistent terminology and naming conventions, correct spelling and grammar, 3-5 sentence paragraphs, use of text formatting (bold, lists, visual dividers, headers, whitespace), optimize for skimmability

GOOD README EXAMPLES: FreeCodeCamp, Bootstrap, React, 
