def latest(scores):
    return scores[-1]


def personal_best(scores):
    return max(scores)


def personal_top_three(scores):
    sorted_scores = sorted(scores, reverse=True)
    if len(sorted_scores) < 3:
        return [x for x in sorted_scores]
    return sorted_scores[0:3]
   
    
