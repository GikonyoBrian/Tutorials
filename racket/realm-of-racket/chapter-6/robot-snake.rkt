#lang racket
(require 2htdp/image 2htdp/universe)


;; Constants ---------------------------------------------------------------------------------------
(define TICK-RATE 1/10)

;; Board
(define SIZE 30)

;; Snake
(define SEG-SIZE 15)

;; Goo
(define MAX-GOO 5)
(define EXPIRATION-TIME 150)

;; Graphical Board
(define WIDTH-PX  (* SEG-SIZE 30))
(define HEIGHT-PX (* SEG-SIZE 30))

;; Visual
(define MT-SCENE (empty-scene WIDTH-PX HEIGHT-PX))
(define GOO-IMG (bitmap "graphics/goo.gif"))
(define SEG-IMG (bitmap "graphics/body.gif"))
(define HEAD-IMG (bitmap "graphics/head.gif"))

(define HEAD-LEFT-IMG HEAD-IMG)
(define HEAD-DOWN-IMG (rotate 90 HEAD-LEFT-IMG))
(define HEAD-RIGHT-IMG (flip-horizontal HEAD-LEFT-IMG))
(define HEAD-UP-IMG (flip-vertical HEAD-DOWN-IMG))

(define ENDGAME-TEXT-SIZE 15)


;; Data --------------------------------------------------------------------------------------------
(struct pit (snake goos) #:transparent)
(struct snake (dir segs) #:transparent)
(struct goo (loc expire) #:transparent)
(struct posn (x y) #:transparent)


;; Main --------------------------------------------------------------------------------------------
(define (start-snake)
  (big-bang (pit (snake "right" (list (posn 1 1)))
                 (list (fresh-goo)
                       (fresh-goo)
                       (fresh-goo)
                       (fresh-goo)
                       (fresh-goo)
                       (fresh-goo)))
            (on-tick next-pit TICK-RATE)
            (on-key direct-snake)
            (to-draw render-pit)
            (stop-when dead? render-end)))


;; Clock Ticks -------------------------------------------------------------------------------------
(define (next-pit w)
  (define snake (pit-snake w))
  (define goos (pit-goos w))
  (define goo-to-eat (can-eat snake goos))
  (if goo-to-eat
      (pit (grow snake) (age-goo (eat goos goo-to-eat)))
      (pit (slither snake) (age-goo goos))))