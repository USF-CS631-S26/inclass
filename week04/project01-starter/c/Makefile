PROG = project01
OBJS = scan.o parse.o eval.o
HEADERS = ntlang.h

#CC=clang
#CFLAGS=-g -gdwarf-4
CC=gcc
CFLAGS=-g

# Pattern rules to avoid explicit rules
%.o : %.c ${HEADERS}
	${CC} ${CFLAGS} -c -o $@ $<

all : ${PROG}

${PROG} : ${PROG}.c ${HEADERS} ${OBJS}
	${CC} ${CFLAGS} -o $@ $< ${OBJS}

clean :
	rm -rf ${PROG} ${OBJS}
	rm -rf ${PROG:=.dSYM}
