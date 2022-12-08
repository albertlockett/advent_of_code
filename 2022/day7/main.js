
const fs = require('fs')
const input = fs.readFileSync('./input.txt').toString()

const lines = input.split('\n')
lines.slice(1, lines.length)

let dirs = newDir('/', null)
let curr = dirs

let allDirs = []
let lilDirs = []



function newDir(name, parent) {
  return {
    name,
    parent,
    size: 0,
    files: {},
    children: {},
  }
}

for (let line of lines) {
  switch (line.charAt(0)) {
    case '$':
      command(line)
      break
    case 'd':
      dir(line)
      break
  }

  if (line.charAt(0) >= '0' && line.charAt(0) <= '9') {
    file(line)
  }
}

populateDirSizes(dirs)
console.log(
  'part 1 ' +
  lilDirs
    .map((({ size }) => size))
    .reduce(function(a,b) {return a + b})
)

allDirs = allDirs
    .map(({ name, size }) => ({ name, size }))
    .sort((a, b) => {
      return (a.size - b.size)
    })

const total  = 70000000
const targu  = 30000000
const usedSize = dirs.size

const delSize = usedSize - (total - targu)
const toDel = allDirs.find(({ size }) => size >= delSize)
console.log(`part 2 ${toDel.size}`)


function command(line) {
  const cmd = line.split(' ')[1]
  switch(cmd) {
    case 'cd':
      nav(line)

  }
}

function nav(line) {
  let [_, __, target] = line.split(' ')
  if (target == '..') {
    curr = curr.parent;
    return;
  }

  if (!curr.children[target]) {
    curr.children[target] = newDir(target, curr)
  }
  curr = curr.children[target]
}

function dir(line) {
  let [_, dirname] = line.split(' ')
  if (!curr[dirname]) {
    curr.children[dirname] = newDir(dirname, curr)
  }
}

function file(line) {
  let [size, name] = line.split(' ')
  size = Number(size)
  curr.files[name] = { size }
}


function populateDirSizes(dir) {
  for (let child of Object.values(dir.children)) {
    populateDirSizes(child)
    dir.size += child.size
  } 
  for (let file of Object.values(dir.files)) {
    dir.size += file.size
  }

  if (dir.size <= 100000) {
    lilDirs.push(dir)
  }

  allDirs.push(dir)
}
