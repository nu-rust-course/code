// From Robby Findler

#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <pthread.h>

/* START: globals */
int x = 0;
int y = 0;
int l = -1;
int r = -1;
/* END: globals */

void* left(void* ThreadArgument)
{
/* START: worker 1 */
    x = 1;
    l = y;
/* END: worker 1 */
    return 0;
}

void* right(void* ThreadArgument)
{
/* START: worker 2 */
    y = 1;
    r = x;
/* END: worker 2 */
    return 0;
}

int main()
{
    pthread_t      t1;
    pthread_t      t2;
    pthread_attr_t attr;
    pthread_attr_init(&attr);
    pthread_attr_setdetachstate
            (&attr, PTHREAD_CREATE_JOINABLE);
    pthread_create(&t1, &attr, left, NULL);
    pthread_create(&t2, &attr, right, NULL);
    pthread_join(t1, NULL);
    pthread_join(t2, NULL);
/* START: output */
    printf("l = %i, r = %i\n", l, r);
/* END: output */
    pthread_attr_destroy(&attr);
    pthread_exit(NULL);
    return 0;
}