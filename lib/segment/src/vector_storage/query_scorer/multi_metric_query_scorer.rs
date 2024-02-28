use std::marker::PhantomData;

use common::types::{PointOffsetType, ScoreType};

use super::score_multivector;
use crate::data_types::vectors::MultiVector;
use crate::spaces::metric::Metric;
use crate::vector_storage::query_scorer::QueryScorer;
use crate::vector_storage::MultiVectorStorage;

pub struct MetricQueryScorer<'a, TMetric: Metric, TVectorStorage: MultiVectorStorage> {
    vector_storage: &'a TVectorStorage,
    query: MultiVector,
    metric: PhantomData<TMetric>,
}

impl<'a, TMetric: Metric, TVectorStorage: MultiVectorStorage>
    MetricQueryScorer<'a, TMetric, TVectorStorage>
{
    #[allow(dead_code)]
    pub fn new(query: MultiVector, vector_storage: &'a TVectorStorage) -> Self {
        Self {
            query: query.into_iter().map(|v| TMetric::preprocess(v)).collect(),
            vector_storage,
            metric: PhantomData,
        }
    }
}

impl<'a, TMetric: Metric, TVectorStorage: MultiVectorStorage> QueryScorer<MultiVector>
    for MetricQueryScorer<'a, TMetric, TVectorStorage>
{
    #[inline]
    fn score_stored(&self, idx: PointOffsetType) -> ScoreType {
        score_multivector::<TMetric>(&self.query, self.vector_storage.get_multi(idx))
    }

    #[inline]
    fn score(&self, v2: &MultiVector) -> ScoreType {
        score_multivector::<TMetric>(&self.query, v2)
    }

    fn score_internal(&self, point_a: PointOffsetType, point_b: PointOffsetType) -> ScoreType {
        let v1 = self.vector_storage.get_multi(point_a);
        let v2 = self.vector_storage.get_multi(point_b);
        score_multivector::<TMetric>(v1, v2)
    }
}
