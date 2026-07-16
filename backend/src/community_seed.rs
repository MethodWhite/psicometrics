use crate::community::types::{Comment, ForumPost, Testimonial, UserStory};
use crate::community::CommunityStore;

pub fn seed_community(store: &CommunityStore) {
    // Only seed if store is empty (check forum posts)
    {
        let posts = store.get_posts(None, 1, 1);
        if !posts.is_empty() {
            return;
        }
    }

    let now = chrono::Utc::now().timestamp();

    // ── 20 Forum Posts ──────────────────────────────────────────────────────

    // Each entry: (personality_type, category, extra_tag, title, content)
    let post_data: Vec<(Option<&str>, &str, Option<&str>, &str, &str)> = vec![
        (Some("INTJ"), "mbti", None, "The INTJ Mastermind: A Deep Dive", "I've been studying the INTJ personality type for years and wanted to share some insights. We're often misunderstood as cold or distant, but there's so much more beneath the surface. Let's discuss our experiences!"),
        (Some("INFP"), "mbti", None, "INFP Creativity and Emotional Depth", "As an INFP, I find that my creativity flows best when I'm emotionally connected to my work. How do other INFPs channel their emotions into creative projects?"),
        (Some("Type 5"), "enneagram", None, "Enneagram Type 5: The Investigator's Journey", "Type 5s are known for their thirst for knowledge. But how do we balance our need for understanding with the demands of daily life?"),
        (None, "general", Some("personality"), "What's your personality type and how has it shaped your career?", "I'm curious how different personality types have influenced career choices. Share your type and your profession!"),
        (Some("ESTJ"), "mbti", None, "ESTJ Leadership in the Workplace", "ESTJs are natural-born leaders. Here are some tips for leveraging your natural organizational skills in team settings while still being approachable."),
        (None, "big_five", Some("openness"), "High Openness: The Double-Edged Sword", "Having high openness to experience is great for creativity but can lead to overthinking. How do you manage this trait?"),
        (Some("ENFP"), "mbti", None, "ENFP: The Campaigner's Social World", "As an ENFP, I thrive on social connections but sometimes struggle with follow-through. Anyone else experience this?"),
        (Some("Type 9"), "enneagram", None, "Type 9 Peacemaking: Strength or Weakness?", "Type 9s are peacemakers, but sometimes our desire for harmony prevents necessary conflict. When should we step up?"),
        (None, "general", None, "How personality tests changed my self-understanding", "After taking the Big Five and MBTI, I finally understood so many patterns in my life. Has anyone else had this revelation?"),
        (Some("ISTJ"), "mbti", None, "ISTJ: The Quiet Dependable Force", "ISTJs often go unnoticed but we're the backbone of many organizations. Let's share our stories of reliability and consistency."),
        (None, "big_five", Some("neuroticism"), "Managing High Neuroticism: Practical Strategies", "High neuroticism can be challenging. Here are evidence-based strategies that have helped me regulate my emotions better."),
        (Some("ENTP"), "mbti", None, "ENTP Debate: How to Argue Constructively", "ENTPs love to debate, but it can come across as confrontational. Tips for channeling our argumentative nature productively."),
        (Some("Type 3"), "enneagram", None, "Type 3 Achievement: Finding Authentic Success", "Type 3s are driven to succeed, but at what cost? Reflections on finding authentic success vs. external validation."),
        (None, "general", Some("relationships"), "Personality compatibility in relationships", "What role should personality types play in choosing partners? My partner and I have opposite types and it works great!"),
        (Some("INFJ"), "mbti", None, "INFJ: The Advocate's Guide to Boundaries", "INFJs are empathetic to a fault. Learning to set boundaries has been transformative. Here's what worked for me."),
        (None, "big_five", Some("conscientiousness"), "Building Conscientiousness: Small Habits Big Impact", "Conscientiousness is the trait most associated with success. Here are micro-habits that helped me become more organized."),
        (Some("ESFJ"), "mbti", None, "ESFJ Community Building Tips", "As an ESFJ, building community comes naturally. Here are my tips for creating welcoming spaces wherever you go."),
        (Some("Type 7"), "enneagram", None, "Type 7: Embracing Stillness in a Busy Mind", "Type 7s chase novelty and experiences. But learning to sit with stillness has been my greatest growth area."),
        (None, "general", Some("self-improvement"), "Using personality insights for personal growth", "The goal of personality testing isn't to box ourselves in, but to understand our patterns. How have you used insights for growth?"),
        (Some("ISFP"), "mbti", None, "ISFP Artistic Expression and Identity", "ISFPs express themselves through art and action. How does your personality type influence your creative expression?"),
    ];

    for (i, (ptype, cat, extra_tag, title, content)) in post_data.iter().enumerate() {
        let mut tags = vec![cat.to_string()];
        if let Some(t) = extra_tag {
            tags.push(t.to_string());
        }
        if let Some(pt) = ptype {
            tags.push(pt.to_lowercase());
        }

        store.create_post(ForumPost {
            id: String::new(),
            author_id: format!("seed_user_{}", i),
            author_name: format!("User_{}", (i + 1)),
            title: title.to_string(),
            content: format!("{}\n\n---\n*This is a sample post for demonstration purposes.*", content),
            category: cat.to_string(),
            personality_type: ptype.map(|s| s.to_string()),
            tags,
            created_at: now - (post_data.len() - i) as i64 * 3600,
            likes: (i as u32 * 3) % 50,
            comment_count: 0,
        });
    }

    // Add some comments to the first post
    let first_post_id = {
        let posts = store.get_posts(None, 1, 1);
        posts.first().map(|p| p.id.clone())
    };

    if let Some(pid) = first_post_id {
        for i in 0..3 {
            store.add_comment(&pid, Comment {
                id: String::new(),
                post_id: pid.clone(),
                author_name: format!("Commenter_{}", i + 1),
                content: format!("Great post! I really enjoyed reading your perspective on this. Thanks for sharing such valuable insights with the community."),
                created_at: now - (3 - i) as i64 * 1800,
            });
        }
    }

    // ── 10 Testimonials ─────────────────────────────────────────────────────

    let testimonials = vec![
        ("María García", "PsicoMetrics helped me understand my anxiety patterns through the Big Five test. The insights were incredibly accurate.", "ENFJ", 5u8),
        ("Carlos López", "The MBTI test was eye-opening. I finally understand why I approach problems the way I do.", "INTJ", 5u8),
        ("Ana Martínez", "Career counseling based on my personality profile helped me make a confident career change.", "ISTP", 4u8),
        ("David Rodríguez", "The Enneagram test revealed patterns I've had since childhood. Truly transformative experience.", "Type 4", 5u8),
        ("Laura Fernández", "I've taken many personality tests, but Psicometrics' reports are the most detailed and useful I've seen.", "INFJ", 5u8),
        ("Jorge Sánchez", "Using this with my partner improved our communication significantly. Highly recommend the compatibility tool.", "ESFJ", 4u8),
        ("Sofia Torres", "The DISC profile helped me understand my leadership style and how to adapt to my team members.", "DISC-D", 4u8),
        ("Miguel Ángel Ruiz", "As a psychologist, I recommend Psicometrics to all my clients. The scientific approach is commendable.", "INTP", 5u8),
        ("Elena Castro", "Finally a personality test that doesn't box you in. The nuanced reports respect human complexity.", "ENFP", 5u8),
        ("Roberto Díaz", "The dark triad assessment was handled with sensitivity and provided valuable self-awareness.", "ISTJ", 4u8),
    ];

    for (name, text, ptype, rating) in testimonials {
        store.add_testimonial(Testimonial {
            id: String::new(),
            name: name.to_string(),
            text: text.to_string(),
            personality_type: ptype.to_string(),
            rating,
            created_at: now - (rand::random::<i64>().abs() % 86400 * 30),
        });
    }

    // ── 5 User Stories ──────────────────────────────────────────────────────

    let stories_data: Vec<(&str, &str, &str, &str, bool, u32)> = vec![
        ("From Burnout to Balance: My Big Five Journey",
         "## The Breaking Point\n\nAfter years of pushing myself to the limit, I hit a wall. My Big Five results showed extremely high conscientiousness and neuroticism — a combination that explained my perfectionism-driven burnout.\n\n## The Insight\n\nUnderstanding my trait profile helped me see that my tendency toward perfectionism wasn't a character flaw — it was a personality trait that needed balanced management rather than elimination.\n\n## The Transformation\n\nI started small: deliberately leaving tasks at \"good enough,\" practicing self-compassion, and building recovery time into my schedule. Six months later, I'm more productive than ever — but without the anxiety that used to accompany it.",
         "Elena Rodríguez", "Big Five", true, 142),
        ("An INTJ's Guide to Emotional Intelligence",
         "## The Stereotype\n\nINTJs are often portrayed as emotionless robots. For years, I leaned into this stereotype, using logic as a shield against vulnerability.\n\n## The Wake-Up Call\n\nA close friend told me, \"I know you care, but no one else can tell.\" That feedback was the catalyst for change.\n\n## The Practice\n\nI started by simply naming my emotions throughout the day. Then I practiced sharing them — first in writing, then verbally. The results transformed my relationships.",
         "Miguel Torres", "INTJ", true, 89),
        ("How the Enneagram Saved My Relationship",
         "## The Conflict\n\nMy partner and I kept having the same argument — I wanted more space, they wanted more connection. We were stuck in a pattern neither of us understood.\n\n## The Discovery\n\nTaking the Enneagram revealed I'm a Type 5 (The Investigator) and my partner is a Type 2 (The Helper). Suddenly our \"personality clash\" made perfect sense.\n\n## The Resolution\n\nWe learned to translate each other's needs: I need autonomy to feel safe; they need appreciation to feel secure. Now we give each other those gifts intentionally.",
         "Ana Lucía Pérez", "Type 5", false, 67),
        ("Finding My Voice as an INFP in Corporate America",
         "## The Mismatch\n\nAs an INFP in a Fortune 500 company, I often felt like a square peg in a round hole. The corporate world rewards assertiveness and quick decisions — my natural strengths are empathy and deep thinking.\n\n## The Pivot\n\nInstead of trying to become someone else, I found my niche: roles that required deep user understanding and long-term vision. My \"soft skills\" became my superpower.\n\n## The Message\n\nYour personality type isn't a limitation — it's a compass pointing toward environments where you'll thrive.",
         "Sofia Kim", "INFP", false, 234),
        ("Type 8's Path to Vulnerable Leadership",
         "## The Armor\n\nAs an Enneagram Type 8, I built my career on strength, decisiveness, and control. Vulnerability felt like weakness.\n\n## The Shift\n\nA mentor challenged me: \"Your team doesn't need you to have all the answers. They need you to be human.\" That reframe changed everything.\n\n## The Result\n\nBy sharing my uncertainties and admitting mistakes, my team's trust in me actually increased. Strength isn't invulnerability — it's the courage to be real.",
         "Carlos Mendoza", "Type 8", true, 178),
    ];

    for (title, content, author, ptype, featured, likes) in stories_data {
        store.add_story(UserStory {
            id: String::new(),
            title: title.to_string(),
            content: content.to_string(),
            author_name: author.to_string(),
            personality_type: ptype.to_string(),
            likes,
            featured,
            created_at: now - (rand::random::<i64>().abs() % 86400 * 60),
        });
    }
}
