import pandas as pd
import re
from sklearn.feature_extraction.text import CountVectorizer, TfidfTransformer
import preprocess
import json


def get_stopwords(stop_file_path):
    with open(stop_file_path, 'r', encoding="utf-8") as f:
        stopwords = f.readlines()
        stop_set = set(m.strip() for m in stopwords)
        return frozenset(stop_set)


def sort(coo_matrix):
    tuples = zip(coo_matrix.col, coo_matrix.data)
    return sorted(tuples, key=lambda x: (x[1], x[0]), reverse=True)


def extract_topn_from_vector(feature_names, sorted_items, topn=10):
    # use only topn items from vector
    sorted_items = sorted_items[:topn]

    score_vals = []
    feature_vals = []

    for idx, score in sorted_items:
        # keep track of feature name and its corresponding score
        score_vals.append(round(score, 3))
        feature_vals.append(feature_names[idx])

    # create a tuples of feature,score
    results = {}
    for idx in range(len(feature_vals)):
        results[feature_vals[idx]] = score_vals[idx]

    return results


data = pd.read_json("data/hn-history.json")
docs = data['text'].tolist()
stopwords = get_stopwords("data/stopwords.txt")

# Rules for vocabulary:
#    1. ignore words that appear in 85% of documents,
#    2. eliminate stop words
#    3. limit vocabulary
cv = CountVectorizer(max_df=0.85, stop_words=stopwords, max_features=10000)
word_count_vector = cv.fit_transform(docs)

# print("Vocabulary: ", list(cv.vocabulary_.keys())[:50])
# print(list(cv.get_feature_names())[2000:2015])

tfidf_transformer = TfidfTransformer(smooth_idf=True, use_idf=True)
tfidf_transformer.fit(word_count_vector)

# get input data
data_in = pd.read_json("../dist/hn.json", lines=True)
data_in = preprocess.combine_text(data_in)
data_in['text'] = data_in['text'].apply(lambda x: preprocess.pre_process(x))

stories = data_in['text'].tolist()
feature_names = cv.get_feature_names()

results = []
for story in stories:
    # generate tf-idf for the given document
    tf_idf_vector = tfidf_transformer.transform(cv.transform([story]))

    # sort the tf-idf vectors by descending order of scores
    sorted_items = sort(tf_idf_vector.tocoo())

    # extract only the top n; n here is 10
    keywords = extract_topn_from_vector(feature_names, sorted_items, 10)

    results.append({
        "text": story,
        "keywords": keywords
    })

out = open("../dist/output.json", "w")
out.write(json.dumps(results))
out.close()

print("keywords saved to dist/output.json")
