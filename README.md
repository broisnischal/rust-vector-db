# Creating the Vector DB in RUST

- Key learning

## What is Vector database?

Vector databases stores unstructured data in the form of vectors that captures the meaning and
context of natural language processing and computer vison. They are designed to efficiently handle the storage and retrival of these dense numerical vectors through specialized data structures and indexing techniques, such as hierachical navigable data world (HNSW) and product
quantization. These databases enable users to find vectors that are most similar to a given query vector based on a chosen distance metric, such as Euclidean distance, cosine similarity, or dot product.

### Vector Embeddings

Vector embeddings are numerical represenatations of unstructured data, such as text, images or audios and even the videos in the form of vectors. These embeddings capture the semantic similarity of objects by mapping them to points in a vector space, where similar objects are represented by vectors that are close to each other.

### Vector Indexing

Vector indexing is a technique used to organize and retrieve data based on vector representations. Instead of storing data in traditional tabular or document formats, vector indices represent data objects as vectors in a multi-dimensional space.

### Distance Metrices

In the context of a vector database, a distance metric refers to a mathematical function that takes two vectors as input and calculates a distance value representing their similarity or dissimilarity. We use three distance measures to gauge the similarity of vectors. Selecting an effective distance measure improves classification and clustering performance.

### LLMs

Large language models (LLMs) are advanced deep-learning models that have been developed to process and analyze human languages. An LLM operates as a highly potent deep-learning model with the capability to comprehend and generate text similar to humans. At its core, this model utilizes a large-scale transformer model to achieve its impressive performance across fields and apps.
