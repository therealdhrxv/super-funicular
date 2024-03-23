let arrived = false;

let a = 0;

const ride = new Promise((resolve, reject) => {
    if (arrived) {
        resolve(
            a += 10
        )
    } else {
        reject(
            a -= 10
        )
    }
})

ride
    .then(value => {
        console.log(value)
    })
    .catch(error => {
        console.log(error)
    })