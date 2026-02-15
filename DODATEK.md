# Dodatek

## DO-178C – standard dla oprogramowania lotniczego (poziomy A–E, A = katastrofalna awaria). Wymaga:

Śledzenia wymagań → kod → testy
Analizy WCET (Worst-Case Execution Time), WCRT (Worst-Case Response Time)
Dowodów deterministyczności działania systemu
Certyfikacji przez FAA/EASA
Dokumentacji procesu rozwoju oprogramowania, w tym szczegółowych planów testów, analizy ryzyka oraz zarządzania konfiguracją
Przeprowadzania przeglądów i audytów na każdym etapie cyklu życia oprogramowania
Weryfikacji i walidacji, aby zapewnić zgodność z wymaganiami bezpieczeństwa i funkcjonalności
Zarządzania zmianami oraz śledzenia wpływu każdej modyfikacji na bezpieczeństwo systemu
Stosowania technik formalnych (opcjonalnie), aby zwiększyć pewność poprawności oprogramowania, szczególnie dla poziomu A

## ARINC 653 – standard architektury awioniki. Wymaga:

Partycjonowanie czasowe i pamięciowe, zapewniające niezależność procesów
Izolacja systemów (np. Flight Control, Navigation, Communication), aby zapobiec wzajemnym zakłóceniom
Gwarancja, że błąd w jednej partycji nie wpłynie na inne, co zwiększa odporność systemu na awarie
Definiowania interfejsów usług systemowych (API), umożliwiających komunikację między partycjami w kontrolowany sposób
Zarządzania zasobami systemowymi, takimi jak procesory, pamięć i urządzenia wejścia/wyjścia, zgodnie z wymaganiami bezpieczeństwa
Obsługi wielu aplikacji o różnych poziomach krytyczności na jednej platformie sprzętowej
Wspierania mechanizmów monitorowania i raportowania błędów w czasie rzeczywistym

## Podsumowanie

Oba wymagają RTOS – bez niego nie ma bezpiecznego lotu. ✈️ RTOS (Real-Time Operating System) zapewnia deterministyczne zarządzanie zadaniami, synchronizację procesów oraz obsługę priorytetów, co jest kluczowe dla spełnienia rygorystycznych wymagań bezpieczeństwa i niezawodności w lotnictwie. RTOS umożliwia implementację partycjonowania, izolacji oraz precyzyjnej kontroli nad czasem wykonania, co pozwala na bezpieczne uruchamianie wielu aplikacji na wspólnej platformie sprzętowej.
