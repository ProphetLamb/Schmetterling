export function tab_textarea(id) {
    var shiftPressed = false;
    var ctrlPressed = false;
    var tabChar = '    ';

    function checkSpecialKeys(e) {
        switch(e.keyCode) {
            case 16:
                shiftPressed = !shiftPressed;
                break;
            case 17:
                ctrlPressed = !ctrlPressed;
        }
    }

    document.addEventListener('keydown', checkSpecialKeys);
    document.addEventListener('keyup', checkSpecialKeys);

    function addTab(textarea) {
        // caching some values, because they are changing as text changes
        var value = textarea.value,
            start = textarea.selectionStart,
            end = textarea.selectionEnd;

        // adding tab character to actual cursor position
        textarea.value = value.substring(0, start) + tabChar + value.substring(end);

        // putting cursor back to its original position
        textarea.selectionStart = textarea.selectionEnd = start + tabChar.length;
    }

    function removeTab(textarea) {
        var curPos = textarea.selectionStart,
            lines = textarea.value.split('\n'),
            newValue = '',
            done = false,
            cnt = 0;

        for (var i = 0, l = lines.length; i < l; i++) {
            // iterating through each line
            var line = lines[i];
            cnt += line.length;
            if (cnt >= curPos && !done) {
                // cursor is in this line
                var newLine = line.replace(new RegExp('^' + tabChar, ''), '');

                if (newLine !== line) {
                    // there was a tab at the beginning of the line, replace was succesfull, cursor must be moved backwards some
                    line = newLine;
                    curPos -=tabChar.length;
                }

                done = true; // only one substitution per run
            }

            newValue += line + '\n';
        }

        // setting new value
        textarea.value = newValue;

        // putting cursor back to its original position
        textarea.selectionStart = textarea.selectionEnd = curPos;
    }

    var textArea = document.getElementById(id);
    textArea.addEventListener('keydown', function(e) {
        if (e.keyCode === 9) {

            if (!shiftPressed) {
                addTab(this);
            } else {
                removeTab(this);
            }

            return false; // preventing losing focus
        }
    });
}