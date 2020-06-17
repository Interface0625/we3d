const fs = require('fs')


fs.readFile('newNumbers.obj', (err, data) => {
    const lines = data.toString().split('\n')
    const objects = []
    const vertices = []
    let currObj;
    lines.forEach(line => {
        const token = line.substring(0, 2).trim()
        switch(token){
            case 'o': objects.push({name: line, v:[], f:[]}); currObj = objects[objects.length-1]; break;
            case 'v': vertices.push(pVertex(line)); break;
            case 'f': currObj.f.push(pFace(line)); break;
            default: break; //console.log('skipping: ' + line)
        }
    })

    const names = []
    let strData = ''
    objects.forEach(obj => {
        const inidces = obj.f.reduce((a,f) => a.concat( f ), [])
        const name = 'number_' + obj.name.slice(2, 3);
        const data = `pub const ${name.toUpperCase()}: [u16; ${inidces.length}] = [ ${inidces.toString()} ] ;`
        console.log(name)
        // fs.writeFile('../src/programs/numbers/'+name+'.rs', data, e=>{})
        strData += data + '\n'
        names.push(name)
    })
    fs.writeFile('../src/programs/numbers/indices.rs', strData, e=>{})

//    const mod_data = names.reduce((a, name) => `${a}mod ${name};
//pub use ${name}::*;
//`, '')
    fs.writeFile('../src/programs/numbers/mod.rs', `
    mod indices;
    pub use indices::*;
    mod vertices;
    pub use vertices::*;
    `, e=>{})

    const finalVertices = vertices.reduce((a, {x,y} )=>a.concat([x, y]), [])
    fs.writeFile('../src/programs/numbers/vertices.rs', `pub const VERTICES: [f32; ${finalVertices.length}] = [ ${finalVertices.toString()} ] ;`, e=>{})



    console.log(objects[4].triangleList)
    //console.log( finalVertices.toString() )
    //console.log(  )
    //fs.writeFile('message.txt', finalVertices, (err) => {
    //    if (err) throw err;
    //    console.log('The file has been saved!');
    //  });
})
 
const pVertex = s => {
    const d = s.split(' ')
    return { x: d[1], y: d[3] }
}
const pFace = s => {
    const d = s.split(' ').slice(1).reduce((a,f) => a.concat( f.split('/') ),[])
    const f = [ 
        parseInt(d[0]) - 1, 
        parseInt(d[3]) - 1, 
        parseInt(d[6]) - 1,
    ]
    return f
}

