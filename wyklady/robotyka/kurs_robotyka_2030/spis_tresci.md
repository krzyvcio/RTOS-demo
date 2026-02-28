# Spis treści kursu (24 wykłady)

## Część 1 (inżynierska, II/III rok): Wstęp do Robotyki — 12 wykładów

1. **Czym jest robotyka dziś? Definicje, klasy robotów, metryki skuteczności** — Przegląd typów robotów (manipulatory, mobilne, humanoidy) oraz kryteriów oceny (dokładność, powtarzalność, bezpieczeństwo, koszt). *(~20 slajdów)*
2. **Matematyka ruchu: wektory, macierze, rotacje, kwaterniony i podstawy algebry Liego** — Narzędzia do opisu orientacji i ruchu w 3D oraz intuicja stojąca za \(SO(3)\), \(SE(3)\). *(~22 slajdy)*
3. **Układy odniesienia i transformacje jednorodne w \(SE(3)\)** — Spójny opis położeń/orientacji, składanie transformacji, interpretacja geometryczna macierzy jednorodnych. *(~20 slajdów)*
4. **Kinematyka prosta manipulatorów: konwencja DH i modele geometryczne** — Wyprowadzanie modelu kinematyki prostej, typowe pułapki, weryfikacja poprawności modelu. *(~22 slajdy)*
5. **Kinematyka odwrotna: metody analityczne i numeryczne, osobliwości** — Rozwiązywanie IK, wielorozwiązaniowość, warunki osobliwości i konsekwencje praktyczne. *(~23 slajdy)*
6. **Prędkości i Jacobian: kinematyka różniczkowa oraz sterowanie w przestrzeni zadaniowej** — Związek prędkości przegubowych i końcówki roboczej, manipulowalność, sterowanie oparte o \(J\). *(~24 slajdy)*
7. **Dynamika robotów: Lagrange/Euler–Newton i znaczenie członów nieliniowych** — Skąd biorą się macierze \(M(q)\), \(C(q,\dot q)\), \(g(q)\) i jak wpływają na sterowanie. *(~24 slajdy)*
8. **Sterowanie klasyczne robotów: PID, sprzężenie od modelu, stabilność i ograniczenia** — Projektowanie regulatorów, nasycenia, opóźnienia, podstawy analizy stabilności w robotyce. *(~22 slajdy)*
9. **Planowanie ruchu: od przestrzeni konfiguracyjnej do RRT\*, PRM i ograniczeń dynamicznych** — Algorytmy planowania, kolizje, koszt trajektorii i podstawy planowania z ograniczeniami. *(~24 slajdy)*
10. **Percepcja dla robotów: czujniki (kamera, LiDAR, IMU), kalibracja i fuzja** — Modele pomiaru, błędy, kalibracja oraz praktyczna fuzja sygnałów. *(~22 slajdy)*
11. **Lokalizacja i mapowanie: podstawy SLAM (EKF/graph-based) i ocena jakości** — Intuicja probabilistyczna, pętle domykające, metryki (ATE/RPE) i ograniczenia w realnym świecie. *(~23 slajdy)*
12. **Systemy robotyczne w praktyce: ROS2, architektury oprogramowania, bezpieczeństwo funkcjonalne** — Wzorce integracji, komunikacja, testowanie, wprowadzenie do standardów i analizy ryzyka. *(~20 slajdów)*

## Część 2 (magisterska/doktorancka): Zaawansowane Problemy Robotyki w Roku 2030 — 12 wykładów

13. **Physical AI i modele fundamentalne dla robotów: od percepcji do działania** — Jak modele uczone na wielkich zbiorach danych zmieniają planowanie, manipulację i uogólnianie zachowań. *(~24 slajdy)*
14. **Uczenie przez naśladownictwo i uczenie ze wzmocnieniem w skali: dane, symulacja, uogólnianie** — Pipeline „dane–polityka–wdrożenie”, problem dystrybucji, ocena i reprodukowalność. *(~24 slajdy)*
15. **Modele świata i planowanie oparte o przewidywanie: MPC, trajektorie w przestrzeni latentnej, VLA/VLM** — Łączenie sterowania predykcyjnego z modelami generatywnymi i ograniczeniami bezpieczeństwa. *(~25 slajdów)*
16. **Cyfrowe bliźniaki i sim2real 2030: identyfikacja, losowość domeny, walidacja** — Metody redukcji luki symulacja–rzeczywistość i „ciągłe” bliźniaki dla utrzymania ruchu. *(~23 slajdy)*
17. **Humanoidy 2030: whole-body control, równowaga, manipulacja dwuręczna i lokomocja** — Sterowanie całym ciałem, kontakt, optymalizacja online i ograniczenia energetyczne w zastosowaniach. *(~25 slajdów)*
18. **Współdzielona autonomia człowiek–robot: zaufanie, nadzór, interfejsy i odpowiedzialność** — Formalizacja podziału decyzyjności, modele człowieka w pętli oraz skutki prawne. *(~22 slajdy)*
19. **Roboty miękkie (soft robotics) i zmienna sztywność: nowe materiały, modelowanie, sterowanie** — Aktuacja, sensoryka miękka, trudności identyfikacji i potencjał w medycynie/przemyśle do 2030. *(~22 slajdy)*
20. **Efektywność energetyczna i aktuatory przyszłości: sprężystość, odzysk energii, napędy hybrydowe** — Projektowanie pod budżet mocy, sprawność w cyklu zadaniowym i konsekwencje dla robotów mobilnych i humanoidów. *(~23 slajdy)*
21. **Roboty zespołowe i roje: koordynacja, odporność, komunikacja, emergencja** — Algorytmy rozproszone, skalowanie do setek jednostek, odporność na awarie i zakłócenia. *(~22 slajdy)*
22. **Roboty w środowiskach krytycznych (kosmos, podwodne, ratownictwo): autonomia i niezawodność** — Projektowanie pod niepewność, degradację czujników i ograniczoną łączność; metody walidacji bezpieczeństwa. *(~23 slajdy)*
23. **Bezpieczeństwo, etyka i regulacje 2030: normy, certyfikacja ML, audyty i zgodność** — Jak wymogi prawne i społeczne ograniczą/ukierunkują robotykę opartą o uczenie maszynowe. *(~20 slajdów)*
24. **Rynek i społeczeństwo 2030+: praca, edukacja, nierówności, geopolityka łańcuchów dostaw robotów** — Realistyczne scenariusze adopcji, koszty wdrożeń, bariery i mierzalne skutki społeczne. *(~20 slajdów)*

