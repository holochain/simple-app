# simple-app

[![Project](https://img.shields.io/badge/project-holochain-blue.svg?style=flat-square)](http://holochain.org/)
[![Chat](https://img.shields.io/badge/chat-chat%2eholochain%2enet-blue.svg?style=flat-square)](https://chat.holochain.net)
[![License: GPL v3](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](http://www.gnu.org/licenses/gpl-3.0)

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

4. Install the [n3h networking component](https://github.com/holochain/n3h)

5. Compile the DNA:

```shell
hc package
```

6. Run two instances of it specifying the install directory for n3h (in the examples below this is `/home/eric/holochain/n3h`). First, in one terminal window, run the first node on port 8888 like this:

```shell
HC_N3H_PATH=/home/eric/holochain/n3h hc run --port 8888
```
Note the network address that is created for the node, you should see something like:

``` shell
READY! tcp://127.0.0.1:40197 ["/ip4/127.0.0.1/tcp/46513/ipfs/Qmd7SXFpgr16kkEVHxmRR1csB8CYwsN79vBtdNvmKQBBwi", "/ip4/192.168.1.5/tcp/46513/ipfs/Qmd7SXFpgr16kkEVHxmRR1csB8CYwsN79vBtdNvmKQBBwi"]
```

Then, in another terminal window (from the same directory), run the second node on port 8889 using a different agent name and the address from the first node as the bootstrap node like this:
```shell
HC_N3H_BOOTSTRAP_NODE=/ip4/192.168.1.5/tcp/46513/ipfs/Qmd7SXFpgr16kkEVHxmRR1csB8CYwsN79vBtdNvmKQBBwi HC_AGENT=testAgent2 HC_N3H_PATH=/home/eric/holochain/n3h  hc run --port 8889
```

7. **Finally to run the UI:** simply open the `ui/index.html` file in a browser, and it should start communicating with the two instances of `hc` via websockets.

## Bugs
Currently n3h process spawned by `hc run` are not automatically killed when the run ends so you may have to kill them manually with `killall node`

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
