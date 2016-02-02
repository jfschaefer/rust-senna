#include <stdlib.h>
#include <stdio.h>
#include <assert.h>

#include "c_wrapper.h"


/* asserts that tokens have been generated */
#define TOKEN_DEF_CHECK() { if (!senna->lastSentence.tokens) { fprintf(stderr,        \
        "rust-senna: Fatal: Tokenization results accessed before generation\n");      \
     exit(EXIT_FAILURE); }  }

/* asserts that pos labels have been generated */
#define POS_DEF_CHECK() { if (!senna->lastSentence.pos_labels) { fprintf(stderr,        \
        "rust-senna: Fatal: POS labels accessed, but not generated\n");                 \
     exit(EXIT_FAILURE); }  }

/* asserts that psg labels have been generated */
#define PSG_DEF_CHECK() { if (!senna->lastSentence.psg_labels) { fprintf(stderr,        \
        "rust-senna: Fatal: PSG labels accessed, but not generated\n");                 \
     exit(EXIT_FAILURE); }  }

/*
 * allocates hash tables labels and provides a pointer to the data structure
 * has to be freed using freeSenna
 */
SENNA* sennaCreate(const char * opt_path) {
    SENNA* senna = (SENNA*) malloc(sizeof(SENNA));
    CHECK_ALLOC(senna);

    senna->word_hash = SENNA_Hash_new(opt_path, "hash/words.lst");
    senna->caps_hash = SENNA_Hash_new(opt_path, "hash/caps.lst");
    senna->suff_hash = SENNA_Hash_new(opt_path, "hash/suffix.lst");
    senna->gazt_hash = SENNA_Hash_new(opt_path, "hash/gazetteer.lst");

    senna->gazl_hash = SENNA_Hash_new_with_admissible_keys(opt_path, "hash/ner.loc.lst", "data/ner.loc.dat");
    senna->gazm_hash = SENNA_Hash_new_with_admissible_keys(opt_path, "hash/ner.msc.lst", "data/ner.msc.dat");
    senna->gazo_hash = SENNA_Hash_new_with_admissible_keys(opt_path, "hash/ner.org.lst", "data/ner.org.dat");
    senna->gazp_hash = SENNA_Hash_new_with_admissible_keys(opt_path, "hash/ner.per.lst", "data/ner.per.dat");

    // labels
    senna->pos_hash = SENNA_Hash_new(opt_path, "hash/pos.lst");
    // senna->chk_hash = SENNA_Hash_new(opt_path, "hash/chk.lst");
    // senna->pt0_hash = SENNA_Hash_new(opt_path, "hash/pt0.lst");
    // senna->ner_hash = SENNA_Hash_new(opt_path, "hash/ner.lst");
    // senna->vbs_hash = SENNA_Hash_new(opt_path, "hash/vbs.lst");
    // senna->srl_hash = SENNA_Hash_new(opt_path, "hash/srl.lst");
    senna->psg_left_hash = SENNA_Hash_new(opt_path, "hash/psg-left.lst");
    senna->psg_right_hash = SENNA_Hash_new(opt_path, "hash/psg-right.lst");

    senna->pos = SENNA_POS_new(opt_path, "data/pos.dat");
    // senna->chk = SENNA_CHK_new(opt_path, "data/chk.dat");
    // senna->pt0 = SENNA_PT0_new(opt_path, "data/pt0.dat");
    // senna->ner = SENNA_NER_new(opt_path, "data/ner.dat");
    // senna->vbs = SENNA_VBS_new(opt_path, "data/vbs.dat");
    // senna->srl = SENNA_SRL_new(opt_path, "data/srl.dat");
    senna->psg = SENNA_PSG_new(opt_path, "data/psg.dat");

    senna->tokenizer = SENNA_Tokenizer_new(
                        senna->word_hash,
                        senna->caps_hash,
                        senna->suff_hash,
                        senna->gazt_hash,
                        senna->gazl_hash,
                        senna->gazm_hash,
                        senna->gazo_hash,
                        senna->gazp_hash,
                        0);

    senna->lastSentence.tokens = NULL;
    senna->lastSentence.pos_labels = NULL;
    senna->lastSentence.psg_labels = NULL;

    senna->strbuf.ptr = (char *) malloc(sizeof(char) * 512);
    CHECK_ALLOC(senna->strbuf.ptr);
    senna->strbuf.length = 512;
    senna->strbuf.pos = 0;

    return senna;
}

void sennaFree(SENNA *senna) {
    SENNA_Hash_free(senna->word_hash);
    SENNA_Hash_free(senna->caps_hash);
    SENNA_Hash_free(senna->suff_hash);
    SENNA_Hash_free(senna->gazt_hash);

    SENNA_Hash_free(senna->gazl_hash);
    SENNA_Hash_free(senna->gazm_hash);
    SENNA_Hash_free(senna->gazo_hash);
    SENNA_Hash_free(senna->gazp_hash);

    SENNA_Hash_free(senna->pos_hash);
    // SENNA_Hash_free(senna->chk_hash);
    // SENNA_Hash_free(senna->pt0_hash);
    // SENNA_Hash_free(senna->ner_hash);
    // SENNA_Hash_free(senna->vbs_hash);
    // SENNA_Hash_free(senna->srl_hash);
    SENNA_Hash_free(senna->psg_left_hash);
    SENNA_Hash_free(senna->psg_right_hash);

    SENNA_POS_free(senna->pos);
    // SENNA_CHK_free(senna->chk);
    // SENNA_PT0_free(senna->pt0);
    // SENNA_NER_free(senna->ner);
    // SENNA_VBS_free(senna->vbs);
    // SENNA_SRL_free(senna->srl);
    SENNA_PSG_free(senna->psg);

    SENNA_Tokenizer_free(senna->tokenizer);

    free(senna->strbuf.ptr);

    free(senna);
}


void sennaParseSentence(SENNA *senna, const char *sentence, unsigned int options) {
    // Tokenize
    SENNA_Tokens *tokens = SENNA_Tokenizer_tokenize(senna->tokenizer, sentence);
    senna->lastSentence.tokens = tokens;
    assert(tokens);

    // Pos
    if (options == GENERATE_POS || options == GENERATE_PSG) {
        senna->lastSentence.pos_labels = SENNA_POS_forward(
                senna->pos, tokens->word_idx, tokens->caps_idx, tokens->suff_idx, tokens->n);
    } else {
        senna->lastSentence.pos_labels = NULL;   // indicate they've not been generated
    }

    // Psg
    if (options == GENERATE_PSG) {
        assert(senna->lastSentence.pos_labels);
        SENNA_PSG_forward(senna->psg, tokens->word_idx, tokens->caps_idx,
                senna->lastSentence.pos_labels, tokens->n, &senna->lastSentence.psg_labels,
                &senna->lastSentence.n_psg_level);

        int is_psg_one_segment = 0;
        int i;
        int n_psg_level = senna->lastSentence.n_psg_level;
        /* The following 15 lines are some magic from SENNA_main.c. */
        /* check if top level takes the full sentence */
        {
            int *psg_top_labels = senna->lastSentence.psg_labels + (n_psg_level-1)*tokens->n;

            if(tokens->n == 1)
                is_psg_one_segment = ((psg_top_labels[0]-1) % 4 == 3); /* S- ? */
            else
                is_psg_one_segment = ((psg_top_labels[0]-1) % 4 == 0) && ((psg_top_labels[tokens->n-1]-1) % 4 == 2); /* B- or E- ? */

            for(i = 1; is_psg_one_segment && (i < tokens->n-1); i++)
            {
                if((psg_top_labels[i]-1) % 4 != 1) /* I- ? */
                    is_psg_one_segment = 0;
            }
        }
        senna->lastSentence.is_psg_one_segment = is_psg_one_segment;
    } else {  // don't generate psg
        senna->lastSentence.psg_labels = NULL;   // indicate they haven't been generated
    }
}



unsigned int sennaGetNumberOfWords(const SENNA *senna) {
    TOKEN_DEF_CHECK();
    return senna->lastSentence.tokens->n;
}


unsigned int sennaGetStartOffset(const SENNA* senna, unsigned int token) {
    TOKEN_DEF_CHECK();
    return senna->lastSentence.tokens->start_offset[token];
}


unsigned int sennaGetEndOffset(const SENNA* senna, unsigned int token) {
    TOKEN_DEF_CHECK();
    return senna->lastSentence.tokens->end_offset[token];
}


const char * sennaGetPOS(const SENNA *senna, unsigned int token) {
    TOKEN_DEF_CHECK();
    POS_DEF_CHECK();
    return SENNA_Hash_key(senna->pos_hash, senna->lastSentence.pos_labels[token]);
}


#define BUFSIZER() if (senna->strbuf.pos == senna->strbuf.length) {           \
    senna->strbuf.length *= 2;                                                \
    senna->strbuf.ptr = (char *) malloc(senna->strbuf.length * sizeof(char)); \
    CHECK_ALLOC(senna->strbuf.ptr);                                           \
  }

#define BUFPRINT(str) {                      \
    const char *tmp = str;                   \
    while (*tmp) {                           \
        senna->strbuf.ptr[senna->strbuf.pos++]  = *tmp++;    \
        BUFSIZER()                           \
    } }


const char * sennaGetPSGStr(SENNA *senna) {
    TOKEN_DEF_CHECK();
    POS_DEF_CHECK();
    PSG_DEF_CHECK();
    int i, j;
    if (!senna->lastSentence.is_psg_one_segment) {
        BUFPRINT("(S");
    }

    struct ParsedSentence sentence = senna->lastSentence;
    for (i = 0; i < sentence.tokens->n; i++) {
        for (j = sentence.n_psg_level - 1; j >= 0; j--) {
            BUFPRINT(SENNA_Hash_key(senna->psg_left_hash, sentence.psg_labels[j*sentence.tokens->n+i]));
        }
        BUFPRINT("*");
        for (j = 0; j < sentence.n_psg_level; j++) {
            BUFPRINT(SENNA_Hash_key(senna->psg_right_hash, sentence.psg_labels[j*sentence.tokens->n+i]));
        }
    }

    if (!senna->lastSentence.is_psg_one_segment) {
        BUFPRINT(")");
    }

    senna->strbuf.ptr[senna->strbuf.pos] = '\0';

    return senna->strbuf.ptr;
}

