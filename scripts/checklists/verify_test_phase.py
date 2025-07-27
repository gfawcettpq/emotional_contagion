#!/usr/bin/env python3
"""
Verification script for Test Phase completion
Compares AI claims against actual project metrics
"""

import json
import os
import subprocess
import glob
from datetime import datetime
from pathlib import Path
import hashlib

def count_bdd_features():
    """Count BDD feature files and scenarios"""
    feature_files = glob.glob("tests/bdd/features/*.feature")
    total_scenarios = 0
    
    for feature_file in feature_files:
        with open(feature_file, 'r') as f:
            content = f.read()
            total_scenarios += content.count('Scenario:')
    
    step_files = glob.glob("tests/bdd/steps/*.rs") + glob.glob("tests/bdd/steps/*.py")
    
    return {
        "feature_files": len(feature_files),
        "scenarios": total_scenarios,
        "step_files": len(step_files)
    }

def count_unit_tests():
    """Count unit test functions and files"""
    test_files = glob.glob("tests/unit/**/*.rs", recursive=True)
    total_test_functions = 0
    mock_objects = 0
    
    for test_file in test_files:
        with open(test_file, 'r') as f:
            content = f.read()
            total_test_functions += content.count('#[test]')
            total_test_functions += content.count('#[rstest]')
            mock_objects += content.count('Mock')
    
    return {
        "total_test_functions": total_test_functions,
        "test_files": len(test_files),
        "mock_objects": mock_objects
    }

def count_integration_tests():
    """Count integration test scenarios"""
    integration_files = glob.glob("tests/integration/**/*.rs", recursive=True)
    integration_scenarios = 0
    component_interactions = 0
    
    for test_file in integration_files:
        with open(test_file, 'r') as f:
            content = f.read()
            integration_scenarios += content.count('#[test]')
            integration_scenarios += content.count('#[tokio::test]')
            component_interactions += content.count('integration')
    
    return {
        "integration_scenarios": integration_scenarios,
        "component_interactions": component_interactions
    }

def count_performance_tests():
    """Count performance benchmarks"""
    bench_files = glob.glob("benches/**/*.rs", recursive=True)
    benchmark_functions = 0
    load_test_scenarios = 0
    
    for bench_file in bench_files:
        with open(bench_file, 'r') as f:
            content = f.read()
            benchmark_functions += content.count('#[bench]')
            benchmark_functions += content.count('criterion::')
            load_test_scenarios += content.count('load_test')
    
    return {
        "benchmark_functions": benchmark_functions,
        "load_test_scenarios": load_test_scenarios
    }

def get_coverage_metrics():
    """Analyze test coverage if available"""
    # This would normally run cargo tarpaulin or similar
    # For now, return placeholder metrics
    return {
        "claimed_coverage_percentage": 0,
        "report_size_lines": 0,
        "uncovered_functions": 0
    }

def verify_ai_claims(claims_file):
    """Main verification function"""
    
    if not os.path.exists(claims_file):
        return {"error": "No AI claims file found"}
    
    with open(claims_file, 'r') as f:
        ai_claims = json.load(f)
    
    # Get actual metrics
    actual_bdd = count_bdd_features()
    actual_unit = count_unit_tests()
    actual_integration = count_integration_tests()
    actual_performance = count_performance_tests()
    actual_coverage = get_coverage_metrics()
    
    # Extract AI claims
    claimed_bdd = ai_claims.get("component_claims", {}).get("bdd_coverage", {})
    claimed_unit = ai_claims.get("component_claims", {}).get("unit_tests", {})
    claimed_integration = ai_claims.get("component_claims", {}).get("integration_tests", {})
    claimed_performance = ai_claims.get("component_claims", {}).get("performance_tests", {})
    claimed_coverage = ai_claims.get("component_claims", {}).get("coverage_report", {})
    
    # Calculate accuracy for each component
    def calculate_accuracy(claimed, actual, metric):
        if metric not in claimed or metric not in actual:
            return None
        if actual[metric] == 0 and claimed[metric] == 0:
            return 100
        if actual[metric] == 0:
            return 0
        return min(100, (min(claimed[metric], actual[metric]) / max(claimed[metric], actual[metric])) * 100)
    
    accuracy_results = {
        "bdd_features_accuracy": calculate_accuracy(claimed_bdd, actual_bdd, "feature_files"),
        "bdd_scenarios_accuracy": calculate_accuracy(claimed_bdd, actual_bdd, "scenarios"),
        "unit_functions_accuracy": calculate_accuracy(claimed_unit, actual_unit, "total_test_functions"),
        "integration_scenarios_accuracy": calculate_accuracy(claimed_integration, actual_integration, "integration_scenarios"),
        "performance_benchmarks_accuracy": calculate_accuracy(claimed_performance, actual_performance, "benchmark_functions")
    }
    
    # Calculate overall accuracy
    valid_accuracies = [acc for acc in accuracy_results.values() if acc is not None]
    overall_accuracy = sum(valid_accuracies) / len(valid_accuracies) if valid_accuracies else 0
    
    # Behavioral analysis
    confidence = ai_claims.get("claim_metadata", {}).get("confidence_level", 0)
    overconfidence = confidence >= 8 and overall_accuracy < 80
    
    # Generate verification ID
    timestamp = datetime.now().isoformat()
    verification_id = f"verify_{datetime.now().strftime('%Y%m%d_%H%M%S')}_{hashlib.md5(timestamp.encode()).hexdigest()[:8]}"
    
    verification_result = {
        "verification_metadata": {
            "verification_id": verification_id,
            "timestamp": timestamp,
            "claims_file": claims_file
        },
        "actual_metrics": {
            "bdd_coverage": actual_bdd,
            "unit_tests": actual_unit,
            "integration_tests": actual_integration,
            "performance_tests": actual_performance,
            "coverage_report": actual_coverage
        },
        "claimed_metrics": {
            "bdd_coverage": claimed_bdd,
            "unit_tests": claimed_unit,
            "integration_tests": claimed_integration,
            "performance_tests": claimed_performance,
            "coverage_report": claimed_coverage
        },
        "accuracy_analysis": {
            "overall_accuracy_percentage": round(overall_accuracy, 1),
            "component_accuracy": accuracy_results
        },
        "behavioral_analysis": {
            "overconfidence_indicator": overconfidence,
            "false_positive_readiness": ai_claims.get("readiness_assessment", {}).get("ready_for_transition", False) and overall_accuracy < 85,
            "metric_inflation": any(
                claimed.get(metric, 0) > actual.get(metric, 0) * 1.2 
                for claimed, actual, metric in [
                    (claimed_bdd, actual_bdd, "feature_files"),
                    (claimed_unit, actual_unit, "total_test_functions"),
                    (claimed_integration, actual_integration, "integration_scenarios")
                ]
            ),
            "accuracy_grade": "A" if overall_accuracy >= 95 else
                           "B" if overall_accuracy >= 85 else  
                           "C" if overall_accuracy >= 75 else
                           "D" if overall_accuracy >= 65 else "F"
        }
    }
    
    # Save verification record
    record_file = f"verification/records/{verification_id}.json"
    os.makedirs("verification/records", exist_ok=True)
    with open(record_file, 'w') as f:
        json.dump(verification_result, f, indent=2)
    
    # Make record immutable
    os.chmod(record_file, 0o444)
    
    return verification_result

if __name__ == "__main__":
    import sys
    claims_file = sys.argv[1] if len(sys.argv) > 1 else "verification/ai_claims.json"
    result = verify_ai_claims(claims_file)
    print(json.dumps(result, indent=2))