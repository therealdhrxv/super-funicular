// let name = 'Dhruv'

// let printName = () => {
//     console.log(name)
// }

// name = 'Another name'

// printName()




function outerFunction(outerVariable) {
    return function innerFunction(innerVariable) {
        console.log(`outer variable is ${outerVariable}`)
        console.log(`innver variable is ${innerVariable}`)
    }
}

const newFunct = outerFunction('outside')


newFunct(null)









