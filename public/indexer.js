


const fs = require('fs');
const glob = require('glob');
const noop = ()=>{};
const ignoreList = ['index.json', 'indexer.js']
glob("**/*.*", {}, function (_, fns) { 
    files = fns.map( file => ignoreList.includes(file) ? undefined : file).filter( v => v != null )
    files.forEach(f => console.log(f));
    fs.writeFile('index.json', JSON.stringify(files, null, 2), noop); 
});