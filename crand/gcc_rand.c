#include <stdint.h>
#include <stdio.h>

#define	TYPE_3		3
#define	BREAK_3		128
#define	DEG_3		31
#define	SEP_3		3



struct random_data
{
  int32_t *fptr;		/* Front pointer.  */
  int32_t *rptr;		/* Rear pointer.  */
  int32_t *state;		/* Array of state values.  */
  int rand_type;		/* Type of random number generator.  */
  int rand_deg;		/* Degree of random number generator.  */
  int rand_sep;		/* Distance between front and rear.  */
  int32_t *end_ptr;		/* Pointer behind state table.  */
};


static int32_t randtbl[DEG_3 + 1] =
{
  TYPE_3,

  -1726662223, 379960547, 1735697613, 1040273694, 1313901226,
  1627687941, -179304937, -2073333483, 1780058412, -1989503057,
  -615974602, 344556628, 939512070, -1249116260, 1507946756,
  -812545463, 154635395, 1388815473, -1926676823, 525320961,
  -1009028674, 968117788, -123449607, 1284210865, 435012392,
  -2017506339, -911064859, -370259173, 1132637927, 1398500161,
  -205601318,
};

static struct random_data unsafe_state =
  {
    .fptr = &randtbl[SEP_3 + 1],
    .rptr = &randtbl[1],
    .state = &randtbl[1],
    .rand_type = TYPE_3,
    .rand_deg = DEG_3,
    .rand_sep = SEP_3,
    .end_ptr = &randtbl[sizeof (randtbl) / sizeof (randtbl[0])]
};






int __random_r (struct random_data *buf, int32_t *result)
{
  int32_t *state;
  state = buf->state;

  int32_t *fptr = buf->fptr;
  int32_t *rptr = buf->rptr;
  int32_t *end_ptr = buf->end_ptr;
  uint32_t val;

  val = *fptr += (uint32_t) *rptr;
  /* Chucking least random bit.  */
  *result = val >> 1;
  ++fptr;
  if (fptr >= end_ptr)
	{
	  fptr = state;
	  ++rptr;
	} else {
	  ++rptr;
	  if (rptr >= end_ptr)
	    rptr = state;
	}
  buf->fptr = fptr;
  buf->rptr = rptr;

  return 0;
}

int __srandom_r (unsigned int seed, struct random_data *buf)
{
  int type;
  int32_t *state;
  long int i;
  int32_t word;
  int32_t *dst;
  int kc;

  type = buf->rand_type;

  state = buf->state;
  /* We must make sure the seed is not 0.  Take arbitrarily 1 in this case.  */
  if (seed == 0)
    seed = 1;
  state[0] = seed;

  dst = state;
  word = seed;
  kc = buf->rand_deg;
  for (i = 1; i < kc; ++i)
    {
      /* This does:
	   state[i] = (16807 * state[i - 1]) % 2147483647;
	 but avoids overflowing 31 bits.  */
      long int hi = word / 127773;
      long int lo = word % 127773;
      word = 16807 * lo - 2836 * hi;
      if (word < 0)
	word += 2147483647;
      *++dst = word;
    }

  buf->fptr = &state[buf->rand_sep];
  buf->rptr = &state[0];
  kc *= 10;
  while (--kc >= 0)
    {
      int32_t discard;
      (void) __random_r (buf, &discard);
    }

 done:
  return 0;
}

long int __random (void)
{
  int32_t retval;
  __random_r(&unsafe_state, &retval);
  return retval;
}

int rand (void)
{
  return (int) __random ();
}

void srand (unsigned int x)
{
  (void) __srandom_r (x, &unsafe_state);
}


int main() {
    srand(1);
    printf("rand: %d %d", rand(), rand());
}
