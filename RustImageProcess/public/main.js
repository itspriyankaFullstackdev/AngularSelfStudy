
async function init(){
    let rustApp = null;
    const input = document.getElementById('upload');
    const reader = new FileReader();
    try{
        rustApp = await import("../pkg");
    }
    catch(e){
        console.error(e);
        return;
    }
    console.info(rustApp);

    reader.onloadend = (()=>{
        const base64 = reader.result.replace(/^data:image\/(png|jpeg|jpg);base64,/ , "");
        let image_data_url = rustApp.grayscale(base64);
        document.getElementById("new-img").setAttribute("src", image_data_url);
    })
    input.addEventListener('change',()=>{
        reader.readAsDataURL(input.files[0]);
    })
}

init();