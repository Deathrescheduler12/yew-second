setInterval(function() {
    let open = document.getElementsByClassName("slide-2")[0];
    let close = document.getElementsByClassName("slide-1")[0];
    
    open.className = "slide slide-1"
    close.className = "slide slide-2"

    console.log("Changed the roles!")
}, 10000)