use crate::storage::Storage;
use crate::{
    Allocator, Bidiagonal, Cholesky, ColPivQR, ComplexField, DefaultAllocator, Dim, DimDiff,
    DimMin, DimMinimum, DimSub, FullPivLU, Hessenberg, Matrix, RealField, Schur, SymmetricEigen,
    SymmetricTridiagonal, LU, QR, SVD, U1, UDU,
};

/// # Rectangular matrix decomposition
///
/// This section contains the methods for computing some common decompositions of rectangular
/// matrices with real or complex components. The following are currently supported:
///
/// | Decomposition            | Factors             | Details |
/// | -------------------------|---------------------|--------------|
/// | QR                       | `Q * R`             | `Q` is an unitary matrix, and `R` is upper-triangular. |
/// | QR with column pivoting  | `Q * R * P⁻¹`       | `Q` is an unitary matrix, and `R` is upper-triangular. `P` is a permutation matrix. |
/// | LU with partial pivoting | `P⁻¹ * L * U`       | `L` is lower-triangular with a diagonal filled with `1` and `U` is upper-triangular. `P` is a permutation matrix. |
/// | LU with full pivoting    | `P⁻¹ * L * U * Q⁻¹` | `L` is lower-triangular with a diagonal filled with `1` and `U` is upper-triangular. `P` and `Q` are permutation matrices. |
/// | SVD                      | `U * Σ * Vᵀ`        | `U` and `V` are two orthogonal matrices and `Σ` is a diagonal matrix containing the singular values. |
impl<N: ComplexField, R: Dim, C: Dim, S: Storage<N, R, C>> Matrix<N, R, C, S> {
    /// Computes the bidiagonalization using householder reflections.
    pub fn bidiagonalize(self) -> Bidiagonal<N, R, C>
    where
        R: DimMin<C>,
        DimMinimum<R, C>: DimSub<U1>,
        DefaultAllocator: Allocator<N, R, C>
            + Allocator<N, C>
            + Allocator<N, R>
            + Allocator<N, DimMinimum<R, C>>
            + Allocator<N, DimDiff<DimMinimum<R, C>, U1>>,
    {
        Bidiagonal::new(self.into_owned())
    }

    /// Computes the LU decomposition with full pivoting of `matrix`.
    ///
    /// This effectively computes `P, L, U, Q` such that `P * matrix * Q = LU`.
    pub fn full_piv_lu(self) -> FullPivLU<N, R, C>
    where
        R: DimMin<C>,
        DefaultAllocator: Allocator<N, R, C> + Allocator<(usize, usize), DimMinimum<R, C>>,
    {
        FullPivLU::new(self.into_owned())
    }

    /// Computes the LU decomposition with partial (row) pivoting of `matrix`.
    pub fn lu(self) -> LU<N, R, C>
    where
        R: DimMin<C>,
        DefaultAllocator: Allocator<N, R, C> + Allocator<(usize, usize), DimMinimum<R, C>>,
    {
        LU::new(self.into_owned())
    }

    /// Computes the QR decomposition of this matrix.
    pub fn qr(self) -> QR<N, R, C>
    where
        R: DimMin<C>,
        DefaultAllocator: Allocator<N, R, C> + Allocator<N, R> + Allocator<N, DimMinimum<R, C>>,
    {
        QR::new(self.into_owned())
    }

    /// Computes the QR decomposition (with column pivoting) of this matrix.
    pub fn col_piv_qr(self) -> ColPivQR<N, R, C>
    where
        R: DimMin<C>,
        DefaultAllocator: Allocator<N, R, C>
            + Allocator<N, R>
            + Allocator<N, DimMinimum<R, C>>
            + Allocator<(usize, usize), DimMinimum<R, C>>,
    {
        ColPivQR::new(self.into_owned())
    }

    /// Computes the Singular Value Decomposition using implicit shift.
    pub fn svd(self, compute_u: bool, compute_v: bool) -> SVD<N, R, C>
    where
        R: DimMin<C>,
        DimMinimum<R, C>: DimSub<U1>, // for Bidiagonal.
        DefaultAllocator: Allocator<N, R, C>
            + Allocator<N, C>
            + Allocator<N, R>
            + Allocator<N, DimDiff<DimMinimum<R, C>, U1>>
            + Allocator<N, DimMinimum<R, C>, C>
            + Allocator<N, R, DimMinimum<R, C>>
            + Allocator<N, DimMinimum<R, C>>
            + Allocator<N::RealField, DimMinimum<R, C>>
            + Allocator<N::RealField, DimDiff<DimMinimum<R, C>, U1>>,
    {
        SVD::new(self.into_owned(), compute_u, compute_v)
    }

    /// Attempts to compute the Singular Value Decomposition of `matrix` using implicit shift.
    ///
    /// # Arguments
    ///
    /// * `compute_u` − set this to `true` to enable the computation of left-singular vectors.
    /// * `compute_v` − set this to `true` to enable the computation of right-singular vectors.
    /// * `eps`       − tolerance used to determine when a value converged to 0.
    /// * `max_niter` − maximum total number of iterations performed by the algorithm. If this
    /// number of iteration is exceeded, `None` is returned. If `niter == 0`, then the algorithm
    /// continues indefinitely until convergence.
    pub fn try_svd(
        self,
        compute_u: bool,
        compute_v: bool,
        eps: N::RealField,
        max_niter: usize,
    ) -> Option<SVD<N, R, C>>
    where
        R: DimMin<C>,
        DimMinimum<R, C>: DimSub<U1>, // for Bidiagonal.
        DefaultAllocator: Allocator<N, R, C>
            + Allocator<N, C>
            + Allocator<N, R>
            + Allocator<N, DimDiff<DimMinimum<R, C>, U1>>
            + Allocator<N, DimMinimum<R, C>, C>
            + Allocator<N, R, DimMinimum<R, C>>
            + Allocator<N, DimMinimum<R, C>>
            + Allocator<N::RealField, DimMinimum<R, C>>
            + Allocator<N::RealField, DimDiff<DimMinimum<R, C>, U1>>,
    {
        SVD::try_new(self.into_owned(), compute_u, compute_v, eps, max_niter)
    }
}

/// # Square matrix decomposition
///
/// This section contains the methods for computing some common decompositions of square
/// matrices with real or complex components. The following are currently supported:
///
/// | Decomposition            | Factors                   | Details |
/// | -------------------------|---------------------------|--------------|
/// | Hessenberg               | `Q * H * Qᵀ`             | `Q` is a unitary matrix and `H` an upper-Hessenberg matrix. |
/// | Cholesky                 | `L * Lᵀ`                 | `L` is a lower-triangular matrix. |
/// | UDU                      | `U * D * Uᵀ`             | `U` is a upper-triangular matrix, and `D` a diagonal matrix. |
/// | Schur decomposition      | `Q * T * Qᵀ`             | `Q` is an unitary matrix and `T` a quasi-upper-triangular matrix. |
/// | Symmetric eigendecomposition | `Q ~ Λ ~ Qᵀ`   | `Q` is an unitary matrix, and `Λ` is a real diagonal matrix. |
/// | Symmetric tridiagonalization | `Q ~ T ~ Qᵀ`   | `Q` is an unitary matrix, and `T` is a tridiagonal matrix. |
impl<N: ComplexField, D: Dim, S: Storage<N, D, D>> Matrix<N, D, D, S> {
    /// Attempts to compute the Cholesky decomposition of this matrix.
    ///
    /// Returns `None` if the input matrix is not definite-positive. The input matrix is assumed
    /// to be symmetric and only the lower-triangular part is read.
    pub fn cholesky(self) -> Option<Cholesky<N, D>>
    where
        DefaultAllocator: Allocator<N, D, D>,
    {
        Cholesky::new(self.into_owned())
    }

    /// Attempts to compute the UDU decomposition of this matrix.
    ///
    /// The input matrix `self` is assumed to be symmetric and this decomposition will only read
    /// the upper-triangular part of `self`.
    pub fn udu(self) -> Option<UDU<N, D>>
    where
        N: RealField,
        DefaultAllocator: Allocator<N, D> + Allocator<N, D, D>,
    {
        UDU::new(self.into_owned())
    }

    /// Computes the Hessenberg decomposition of this matrix using householder reflections.
    pub fn hessenberg(self) -> Hessenberg<N, D>
    where
        D: DimSub<U1>,
        DefaultAllocator: Allocator<N, D, D> + Allocator<N, D> + Allocator<N, DimDiff<D, U1>>,
    {
        Hessenberg::new(self.into_owned())
    }

    /// Computes the Schur decomposition of a square matrix.
    pub fn schur(self) -> Schur<N, D>
    where
        D: DimSub<U1>, // For Hessenberg.
        DefaultAllocator: Allocator<N, D, DimDiff<D, U1>>
            + Allocator<N, DimDiff<D, U1>>
            + Allocator<N, D, D>
            + Allocator<N, D>,
    {
        Schur::new(self.into_owned())
    }

    /// Attempts to compute the Schur decomposition of a square matrix.
    ///
    /// If only eigenvalues are needed, it is more efficient to call the matrix method
    /// `.eigenvalues()` instead.
    ///
    /// # Arguments
    ///
    /// * `eps`       − tolerance used to determine when a value converged to 0.
    /// * `max_niter` − maximum total number of iterations performed by the algorithm. If this
    /// number of iteration is exceeded, `None` is returned. If `niter == 0`, then the algorithm
    /// continues indefinitely until convergence.
    pub fn try_schur(self, eps: N::RealField, max_niter: usize) -> Option<Schur<N, D>>
    where
        D: DimSub<U1>, // For Hessenberg.
        DefaultAllocator: Allocator<N, D, DimDiff<D, U1>>
            + Allocator<N, DimDiff<D, U1>>
            + Allocator<N, D, D>
            + Allocator<N, D>,
    {
        Schur::try_new(self.into_owned(), eps, max_niter)
    }

    /// Computes the eigendecomposition of this symmetric matrix.
    ///
    /// Only the lower-triangular part (including the diagonal) of `m` is read.
    pub fn symmetric_eigen(self) -> SymmetricEigen<N, D>
    where
        D: DimSub<U1>,
        DefaultAllocator: Allocator<N, D, D>
            + Allocator<N, DimDiff<D, U1>>
            + Allocator<N::RealField, D>
            + Allocator<N::RealField, DimDiff<D, U1>>,
    {
        SymmetricEigen::new(self.into_owned())
    }

    /// Computes the eigendecomposition of the given symmetric matrix with user-specified
    /// convergence parameters.
    ///
    /// Only the lower-triangular part (including the diagonal) of `m` is read.
    ///
    /// # Arguments
    ///
    /// * `eps`       − tolerance used to determine when a value converged to 0.
    /// * `max_niter` − maximum total number of iterations performed by the algorithm. If this
    /// number of iteration is exceeded, `None` is returned. If `niter == 0`, then the algorithm
    /// continues indefinitely until convergence.
    pub fn try_symmetric_eigen(
        self,
        eps: N::RealField,
        max_niter: usize,
    ) -> Option<SymmetricEigen<N, D>>
    where
        D: DimSub<U1>,
        DefaultAllocator: Allocator<N, D, D>
            + Allocator<N, DimDiff<D, U1>>
            + Allocator<N::RealField, D>
            + Allocator<N::RealField, DimDiff<D, U1>>,
    {
        SymmetricEigen::try_new(self.into_owned(), eps, max_niter)
    }

    /// Computes the tridiagonalization of this symmetric matrix.
    ///
    /// Only the lower-triangular part (including the diagonal) of `m` is read.
    pub fn symmetric_tridiagonalize(self) -> SymmetricTridiagonal<N, D>
    where
        D: DimSub<U1>,
        DefaultAllocator: Allocator<N, D, D> + Allocator<N, DimDiff<D, U1>>,
    {
        SymmetricTridiagonal::new(self.into_owned())
    }
}
