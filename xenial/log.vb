600 REM print maze
610 REM clear the screen
620 CLS
630 LOCATE 2,1 : REM top left corner of the maze
631 LET player_row = PEEK(1)
632 LET player_col = PEEK(2)
640 FOR row = 0 TO height-1
641   LET line$ = ""
650   FOR col = 0 TO width-1
660     IF player_row = row AND player_col = col THEN
670       REM draw player
680       LET char$ = "@";
690     ELSE
710       LET cell = PEEK(16 + row * width + col)
720       IF cell = 0 THEN LET char$ = " ";
730       IF cell = 1 THEN LET char$ = "X";
740     END IF
741     LET line$ = line$ + char$;
750   NEXT col
760   PRINT line$,
770 NEXT row
780 REM print some statistics on top
790 LOCATE 1, 10 : PRINT "STEPS: "; steps
800 steps = steps + 1
810 RETURN

---

600 REM print maze
610 REM clear the screen
620 CLS
630 LOCATE 2,1 : REM top left corner of the maze
631 LET player_row = PEEK(1)
632 LET player_col = PEEK(2)
640 FOR row = 0 TO height-1
641   LET line$ = ""
650   FOR col = 0 TO width-1
660     IF player_row = row AND player_col = col THEN
670       REM draw player
680       LET char$ = "@";
690     ELSE
710       LET cell = PEEK(16 + row * width + col)
720       IF cell = 0 THEN LET char$ = " ";
730       IF cell = 1 THEN LET char$ = "X";
740     END IF
741     LET line$ = line$ + char$;
750   NEXT col
760   PRINT line$,
770 NEXT row
780 REM print some statistics on top
790 LOCATE 1, 10 : PRINT "STEPS: "; steps
800 steps = steps + 1
810 RETURN