<script language = "javascript">
    function on(name) {
        var elements = document.getElementsByClassName(name);
        for (var i = 0; i < elements.length; i++) {
            var elem = elements[i];
            elem.style.backgroundColor = "pink";
        }
    }
    function off(name) {
        var elements = document.getElementsByClassName(name);
        for (var i = 0; i < elements.length; i++) {
            var elem = elements[i];
            elem.style.backgroundColor = "white";
        }
    }
    </script>

