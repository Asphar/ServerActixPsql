function addNewProfile() {
    var newProfileName = document.forms["formulary"]["newprofile"].value;
    var addHTMLProfile = document.getElementById("addprofile");

    if(newProfileName === "") {
        alert("Type a name of the interface");
    } else {        
            addHTMLProfile.innerHTML += "<input type=\"radio\" name=\"profile\" id=" + newProfileName + " value=" + newProfileName + " /><label>" + newProfileName + "</label><span class=\"espace\"></span>";
    }
}

function downloadConfigurationFile() {
    var interfaceName = document.forms["formulary"]["interface_name"].value;
    var privateKey = document.forms["formulary"]["private_key"].value;
    var element = document.createElement("a");

    var filename = interfaceName + ".conf";
    var contentFile = "[Interface]\nPrivateKey = " + privateKey + "\nListenPort = 51820\nAddress = 10.10.1.1/24\nDNS = 10.43.0.10  \n\n[Peer]\nEndpoint = 10.42.8.2\nAllowedIPs = 0.0.0.0/0\nPublicKey = gSPRSO1X+tOZMS4L5w/ze/XOeAM6V2KCZM94Waj4kEo=";

    if(interfaceName === "" || privateKey === "") {
        alert("fill in all the forms");
    } else {
        element.setAttribute("href", "data:text/plain;charset=utf-8," + encodeURIComponent(contentFile));
        element.setAttribute("download", filename);

        element.style.display = 'none';
        document.body.appendChild(element);

        element.click();

        document.body.removeChild(element);
    }
    
}

