To see the networking bug...

Make sure you're up to date on n3h.

Modify the following line in container-config.toml AND container-config-2.toml, to point to your n3h path
n3h_path = "/Users/connor/code/holochain/n3h"

In a terminal, run

`holochain_container -c ./container-config.toml`

Note the bootstrap node from the debug logs, should look like:
/ip4/192.168.1.182/tcp/36497/ipfs/QmQxMSMCtetqXbSbANHCahQ6iUTYbrURcT7r6ZHnaRNYbH

Add that to the following line of `container-config-2.toml`
bootstrap_nodes = ["/ip4..."]

In another terminal, now run
`holochain_container -c ./container-config-2.toml`

In browsers, open up
/ui/index.html and
/ui2/index.html

In one, create an entry, note the address

In the other, attempt to get the entry at the address. The entry won't be found.

we've identified through debugging that the issue where, with two nodes, one fails to 'hold' an entry that has been gossiped to it, this is what we know (condensed):
it enters the hold_entry_workflow, but doesn't proceed past this line: https://github.com/holochain/holochain-rust/blob/develop/core/src/workflows/hold_entry.rs#L24
It goes through reduce_get_validation_package, and eventually ends up calling this function, that is the deepest that I could get a println!, in net_connection_thread
https://github.com/holochain/holochain-rust/blob/develop/net_connection/src/net_connection_thread.rs#L27

It does not seem that the message actually gets dispatched though. At least, the other node does not show any signs of receiving a request_validation_package message

We did a bunch of debugging to get as far as we did, which I pushed my debugging statements to this branch: 
https://github.com/holochain/holochain-rust/commit/639807497e0371e5a77355123aa7ff764f7a6905

Which, if you rebuild your container binary, you will get the debugging logs




# simple-app

[![Project](https://img.shields.io/badge/project-holochain-blue.svg?style=flat-square)](http://holochain.org/)
[![Chat](https://img.shields.io/badge/chat-chat%2eholochain%2enet-blue.svg?style=flat-square)](https://chat.holochain.net)
[![License: GPL v3](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](http://www.gnu.org/licenses/gpl-3.0)


HC_AGENT=testAgentPhil HC_N3H_BOOTSTRAP_NODE=/ip4/192.168.1.182/tcp/36497/ipfs/QmQxMSMCtetqXbSbANHCahQ6iUTYbrURcT7r6ZHnaRNYbH HC_N3H_PATH=/Users/philipbeadle/holochain/Holochain/n3h hc run -p 8888


***Simple app for doing multi-node testing**

## Overview

This app was built to do multi-node testing of the same DNA from one user interface.

## Install

1. Install the Holochain command line dev tool by following the instructions here: https://developer.holochain.org/start.html

2. Clone this repo:
```shell
    git clone https://github.com/holochain/simple-app
```

3. Make sure things are working by running the tests:

```shell
cd simple-app
hc test
```

4. Compile the DNA and run two instances of it. First, in one terminal window, run:

```shell
hc run --port 8888 --package
```

Then, in another terminal window, run:
```shell
hc run --port 8889
```

Finally to run the UI, simply open the `ui/index.html` file in a browser, and it should start communicating with the two instances of `hc` via websockets.

## Contribute
Holochain is an open source project.  We welcome all sorts of participation and are actively working on increasing surface area to accept it.  Please see our [contributing guidelines](https://github.com/holochain/org/blob/master/CONTRIBUTING.md) for our general practices and protocols on participating in the community.

## License
[![License: GPL v3](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](http://www.gnu.org/licenses/gpl-3.0)

Copyright (C) 2019, Holochain Trust

This program is free software: you can redistribute it and/or modify it under the terms of the license p
rovided in the LICENSE file (GPLv3).  This program is distributed in the hope that it will be useful, bu
t WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
 PURPOSE.

**Note:** We are considering other 'looser' licensing options (like MIT license) but at this stage are using GPL while we're getting the matter sorted out.  See [this article](https://medium.com/holochain/licensing-needs-for-truly-p2p-software-a3e0fa42be6c) for some of our thinking on licensing for distributed application frameworks.
