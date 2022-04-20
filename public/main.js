async function init() {
    let rustApp = null
    try {
        rustApp = await import('../pkg')
    } catch (e){
        console.error(e)
        return
    }
    const input = document.querySelector('#upload')
    const fileReader = new FileReader()
    fileReader.onloadend = () =>{

        let b64encodedImage = fileReader.result.replace(/^data:image\/(png|jpeg|jpg);base64,/, "")
        b64encodedImage = rustApp.greyscale(b64encodedImage)
        document.querySelector("#new-img").setAttribute('src', b64encodedImage)
    }

    input.addEventListener('change', (event) =>{
        fileReader.readAsDataURL(input.files[0])
    })
}
init()