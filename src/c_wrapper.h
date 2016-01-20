#include <SENNA_utils.h>
#include <SENNA_Hash.h>
#include <SENNA_Tokenizer.h>

#include <SENNA_POS.h>
#include <SENNA_CHK.h>
#include <SENNA_NER.h>
#include <SENNA_VBS.h>
#include <SENNA_PT0.h>
#include <SENNA_SRL.h>
#include <SENNA_PSG.h>

/*
 * For the processing argument we need to specify what we want to generate.
 * We'll use bit patterns for that.
 */
#define TOKENIZE_ONLY 0  /* We have to do this step in any case */
#define GENERATE_POS  1  /* Part-of-speech (required for psg) */
#define GENERATE_PSG  2  /* phrase structure */


/*
 * Stores the information obtained after parsing a sentence
 */

struct ParsedSentence {
      SENNA_Tokens* tokens;
      int *pos_labels;
      int *psg_labels;
      int n_psg_level;
      int is_psg_one_segment;
};


/*
 * A struct storing all the hash tables and labels senna needs
 */

struct SENNA_ {
    SENNA_Hash *word_hash;
    SENNA_Hash *caps_hash;
    SENNA_Hash *suff_hash;
    SENNA_Hash *gazt_hash;

    SENNA_Hash *gazl_hash;
    SENNA_Hash *gazm_hash;
    SENNA_Hash *gazo_hash;
    SENNA_Hash *gazp_hash;

    // labels
    SENNA_Hash *pos_hash;
    // SENNA_Hash *chk_hash;
    // SENNA_Hash *pt0_hash;
    // SENNA_Hash *ner_hash;
    // SENNA_Hash *vbs_hash;
    // SENNA_Hash *srl_hash;
    SENNA_Hash *psg_left_hash;
    SENNA_Hash *psg_right_hash;

    SENNA_POS *pos;
    // SENNA_CHK *chk;
    // SENNA_PT0 *pt0;
    // SENNA_NER *ner;
    // SENNA_VBS *vbs;
    // SENNA_SRL *srl;
    SENNA_PSG *psg;

    SENNA_Tokenizer *tokenizer;

    struct ParsedSentence lastSentence;
};


typedef struct SENNA_ SENNA;



/*
 * allocates hash tables labels and provides a pointer to the data structure
 * has to be freed using freeSenna
 */
SENNA* createSenna(const char * opt_path);

void freeSenna(SENNA *senna);

void parseSentence(SENNA *senna, const char *sentence, unsigned int options);

int getNumberOfWords(const SENNA *senna);

