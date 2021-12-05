REM start of subroutine
'print maze
'clear the screen
LET player_row = PEEK(1)
LET player_col = PEEK(2)

IF steps = 0 THEN
    CLS
    LET start_col = 0
    LET end_col  = width - 1
    LET start_row = 0
    LET end_row = height - 1
ELSE
    LET start_col = player_col - 1
    LET end_col = player_col + 1
    IF player_col = 0 THEN
        start_col = 0;
    END IF
    IF player_col = width - 1 THEN
        end_col = width-1;
    END IF
    LET start_row = player_row - 1
    LET end_row = player_row + 1
    IF player_row = 0 THEN
        start_row = 0;
    END IF
    IF player_row = height - 1 THEN
        end_row = height-1;
    END IF
END IF

FOR row = start_row TO end_row
    LOCATE 2+row,1+col
    LET line$ = ""
    FOR col = start_col TO end_col
        IF player_row = row AND player_col = col THEN
        'draw player
            LET char$ = "@";
        ELSE
            LET cell = PEEK(16 + row * width + col)
            IF cell = 0 THEN LET char$ = " ";
            IF cell = 1 THEN LET char$ = "X";
        END IF
        LET line$ = line$ + char$;
    NEXT col
    PRINT line$;
NEXT row

'print some statistics on top
LOCATE 1, 10 : PRINT "STEPS: "; steps
steps = steps + 1
RETURN