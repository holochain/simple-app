// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const test = require('tape');

const { Config, Container } = require("../../holochain-rust/nodejs_container")

//const dnaPath = "./dist/app_spec.hcpkg"
const dnaPath = "./dist/bundle.json"

// closure to keep config-only stuff out of test scope
const container = (() => {
  const agentAlice = Config.agent("alice")

  const dna = Config.dna(dnaPath)

  const instanceAlice = Config.instance(agentAlice, dna)

  const containerConfig = Config.container(instanceAlice)
  return new Container(containerConfig)
})()

// Initialize the Container
container.start()

const alice = container.makeCaller('alice', dnaPath)

test('description of example test', (t) => {
  // Make a call to a Zome function
  // indicating the capability and function, and passing it an input
    const addr = alice.call("simple", "main", "share_item", {"item" : {"content":"sample content"}})

    const result = alice.call("simple", "main", "get_item", {"address": addr.Ok})

  // check for equality of the actual and expected results
  t.deepEqual(result, { Ok: { App: [ 'item', '{"content":"sample content"}' ] } })

  // ends this test
  t.end()
})
