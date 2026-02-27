# Wyklad 7: Roadmapa implementacji (od modelu do systemu wielowarstwowego)

## Cel
Przelozyc architekture na plan pracy, ktory minimalizuje ryzyko:
- najpierw model i symulacja,
- potem petla RT i pomiary,
- potem magistrala,
- potem nadzor i digital twin,
- na koncu safety i testy awaryjne jako standard.

## Kroki (praktycznie)
1. Model ODE w Python/MATLAB: `J*domega/dt = tau - load - B*omega`.
2. Projekt regulatora (PI/PID) + saturacje + anti-windup, testy scenariuszy.
3. Prototyp W2 na Linux PREEMPT_RT: jeden watek SCHED_FIFO, pinning, mlockall, pomiar jitteru.
4. Integracja EtherCAT master (biblioteka) i pierwszy slave.
5. Dodanie W3 (nadzor) i IPC (shared mem + ring).
6. Digital twin: porownanie model vs rzeczywistosc (metryki, baseline).
7. Safety: watchdog, safe stop, fault injection, testy regresji.

## Artefakty, ktore musza powstac
- scenariusze regresji (zaklocenie, saturacja, dropout, opoznienie)
- metryki i progi (p99 rt_loop, licznik missed deadline, piki FFT)
- log format (staly, wersjonowany)

## Checklisty
- Kazdy krok ma metryke sukcesu (nie "dziala u mnie").
- Zmiany wprowadzane sa z regresja scenariuszy.

## Wersja dla komorki: robot + modul procesu
Minimalny workflow (koncept):
1. robot pobiera probke i potwierdza interlock,
2. modul procesu przechodzi READY->RUNNING,
3. robot odchodzi do strefy bezpiecznej,
4. modul procesu publikuje zdarzenia (ALARM/COMPLETE),
5. robot odbiera wynik i kontynuuje.

Wniosek: orkiestrator to soft-RT, ale reakcje awaryjne sa lokalne w modulach.

## Pytania do studentow
1. Jakie metryki uznasz za "definition of done" dla kazdego etapu roadmapy?
2. Jak zaprojektujesz scenariusze regresji, zeby wykryly regresje po zmianie filtru/regulatora?
3. Kiedy przechodzisz z SIL do HIL i co ma byc kryterium tej decyzji?
4. Jak zaplanujesz integracje safety tak, by nie byla "ostatnim sprintem"?

## Projekty studenckie
- "Roadmap tracker": repo z etapami + metrykami + automatycznym raportem regresji.
- "Scenario pack": paczka scenariuszy (delay/dropout/saturation/load step) + porownanie baseline.
- "Workflow orchestrator": prototyp orkiestratora robot+modul z FSM i eventami.

## BONUS
- Najwiecej czasu traci sie na integracji bez kontraktow i bez scenariuszy. Zrob kontrakty i scenariusze najpierw, a integracja stanie sie przewidywalna.
