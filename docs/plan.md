\begin{table}[!ht]
\centering
\caption{Conjuntos de dados e caracterização das topologias de entrada.}
\label{tab:datasets}
\small
\begin{tabular}{ll}
\hline
\textbf{Topologia} & \textbf{Descrição do Padrão Estrutural} \\
\hline
Aleatório & Permutação uniforme de $[0,n{-}1]$ via \texttt{shuffle} com \texttt{ChaCha8Rng} (seed $42\oplus n\oplus\text{tipo}$) \\
Tartarugas & $a[i]=n+i$ para $i<n/2$; $a[i]=i\bmod 10$ para $i\ge n/2$ (metade superior elevada e crescente; metade inferior baixa e cíclica) \\
Zigue-zague & $a[i]=i$ se $i$ par, $a[i]=n-i$ se $i$ ímpar (alternância elemento a elemento; determinístico) \\
Quase ordenado & $[0,n{-}1]$ ordenado com $n/100$ trocas de pares uniformes $(i,j)$, $i\neq j$ \\
Duplicados & $a[i]\sim\mathcal{U}\{0,1,2\}$ i.i.d.\ via \texttt{rng.gen\_range(0..3)} (sem agrupamento espacial intencional) \\
Invertido & $a[i]=n-i$ (ordem estritamente decrescente; pior caso determinístico) \\
\hline
\end{tabular}
\end{table}